use crate::{
    app_state::AppState,
    command_queue::{
        command_queue::CommandQueue, design_command::DesignCommand, design_event::DesignEvent,
    },
    renderer::design_renderer::DesignRenderer,
    utils::wasm::get_canvas_element,
};
use gloo_events::EventListener;
use slotmap::Key;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, MouseEvent, ResizeObserver, ResizeObserverEntry, console,
    js_sys::{self, Function},
};

#[wasm_bindgen]
struct DesignEngine {
    state: Rc<RefCell<AppState>>,
    /// The WebGPU/Vello renderer for design documents.
    /// We use `RefCell` here to prevent issues with async fns.
    renderer: Rc<RefCell<Option<DesignRenderer>>>,
    queue: Rc<RefCell<CommandQueue>>,
    /// Numeric flag (aka "Monotonic token") used to discard stale renderer
    create_generation: RefCell<u64>,
    // Event listeners (here to drop later)
    listeners: RefCell<Vec<EventListener>>,
    resize_observer: RefCell<
        Option<(
            web_sys::ResizeObserver,
            wasm_bindgen::closure::Closure<dyn FnMut(js_sys::Array)>,
        )>,
    >,
    // Communication channel to frontend (also here to drop)
    frontend_callback: Rc<RefCell<Option<Function>>>,
}

#[wasm_bindgen]
impl DesignEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        let state = AppState::new(None, None);
        let queue = CommandQueue::new();

        Ok(DesignEngine {
            state: Rc::new(RefCell::new(state)),
            renderer: Rc::new(RefCell::new(None)),
            queue: Rc::new(RefCell::new(queue)),
            create_generation: RefCell::new(0),
            listeners: RefCell::new(Vec::new()),
            resize_observer: RefCell::new(None),
            frontend_callback: Rc::new(RefCell::new(None)),
        })
    }

    /// Handle commands from frontend (e.g. create layer)
    /// There are 2 types of commands, mutable and response.
    /// Response commands will receive a JSON response with data.
    /// Otherwise mutable commands are queued and receive a simple "true" in JSON.
    #[wasm_bindgen]
    pub fn dispatch(&mut self, cmd_js: JsValue) -> Result<JsValue, JsError> {
        // Convert JS object directly to Rust Enum
        let cmd = serde_wasm_bindgen::from_value::<DesignCommand>(cmd_js)
            .map_err(|_| JsError::new("Not a proper command. Maybe mispelled?"))?;
        match cmd {
            // Response commands
            // These commands expect an immediate response and only read state
            DesignCommand::FetchLayersByParent { parent_id } => {
                let response = self
                    .state
                    .borrow_mut()
                    .layers_service
                    .fetch_layers_by_parent(parent_id);
                response
            }
            DesignCommand::AddLayer { layer } => {
                let key = self.state.borrow_mut().layers_service.add_layer(layer);
                Ok(JsValue::from(key))
            }
            DesignCommand::SelectLayer { layer_id } => {
                self.state
                    .borrow_mut()
                    .layers_service
                    .select_layer(layer_id);
                Ok(JsValue::from_bool(true))
            }
            DesignCommand::UpdateLayer { layer_id, data } => {
                self.state
                    .borrow_mut()
                    .layers_service
                    .update_layer(layer_id, data);
                Ok(JsValue::from_bool(true))
            } // Mutable queue
              // These commands are queued and run before scene is compiled and rendered.
              // _ => {
              //     self.queue.borrow_mut().add(cmd, false);
              //     Ok(JsValue::from_bool(true))
              // }
        }
    }

    /// Creates renderer (`DesignRenderer`) using provided canvas ID.
    /// Also handles clearing old renderer if this method run multiple times (e.g. React StrictMode).
    /// Keep in mind we only allow for 1 renderer and 1 canvas at a time for now.
    #[wasm_bindgen]
    pub async fn create_render(&self, canvas_id: &str) -> Result<JsValue, JsError> {
        // Stop and drop any renderer that is currently attached before
        // creating a new one. This prevents two WebGPU devices from
        // targeting the same canvas (StrictMode / remount).
        {
            let mut renderer = self.renderer.borrow_mut();
            if let Some(existing) = renderer.as_mut() {
                existing.stop();
            }
            *renderer = None;
        }

        // Claim a generation token for this creation attempt.
        let generation = {
            let mut generation_index = self.create_generation.borrow_mut();
            *generation_index += 1;
            *generation_index
        };

        let mut new_renderer = DesignRenderer::create(canvas_id)
            .await
            .map_err(|error_msg| JsError::new(&error_msg.to_string()))?;

        // If a newer creation has started in the meantime, discard this one
        // so it cannot overwrite the newer renderer.
        if *self.create_generation.borrow() != generation {
            new_renderer.stop();
            return Ok(JsValue::from_bool(false));
        }

        // Get the canvas size to update the app state.
        if let Ok((width, height)) = new_renderer.get_canvas_size() {
            self.state
                .borrow_mut()
                .set_width_and_height(width as u16, height as u16);
        };

        // Set the user's device resolution
        {
            self.state
                .borrow_mut()
                .set_resolution(new_renderer.get_canvas_resolution())
        }

        // Store renderer locally directly in mutable pointer
        // (avoids `&mut self` which causes race conditions with other methods).
        *self.renderer.borrow_mut() = Some(new_renderer);

        // Attach event listeners to event targets (like checking window for "onmousemove")
        self.create_event_listeners()?;

        Ok(JsValue::from_bool(true))
    }

    /// Starts render loop
    #[wasm_bindgen]
    pub fn start_renderer(&self) -> Result<JsValue, JsError> {
        let mut renderer_result = self.renderer.borrow_mut();
        match renderer_result.as_mut() {
            Some(renderer) => {
                renderer.run(self.state.clone(), self.queue.clone());
                Ok(JsValue::from_bool(true))
            }
            None => Err(JsError::new("No renderer found, create one first")),
        }
    }

    /// Stops render loop
    #[wasm_bindgen]
    pub fn stop_renderer(&self) -> Result<JsValue, JsError> {
        let mut renderer_result = self.renderer.borrow_mut();
        if let Some(renderer) = renderer_result.as_mut() {
            renderer.stop();
        }
        // Always true since we handle cleanup during creation
        Ok(JsValue::from_bool(true))
    }

    /// Stores a callback from frontend that backend will use to emit events (`DesignEvent`).
    #[wasm_bindgen]
    pub fn setup_communication_callback(&self, callback: Function) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = Some(callback);
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't set callback, memory is locked")),
        }
    }

    /// Stores a callback from frontend that backend will use to emit events (`DesignEvent`).
    #[wasm_bindgen]
    pub fn stop_communication_callback(&self) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = None;
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't set callback, memory is locked")),
        }
    }

    /// Attaches event listeners to window and canvas for mouse events.
    /// Listeners are stored so they can be dropped later which stops them (aka "detaches").
    fn create_event_listeners(&self) -> Result<(), JsError> {
        let window = web_sys::window().ok_or(JsError::new(""))?;
        let canvas = self
            .renderer
            .borrow_mut()
            .as_ref()
            .ok_or(JsError::new("Renderer is not initialized"))?
            .get_canvas()
            .ok_or(JsError::new("Canvas could not be retrieved"))?;

        // Mouse move
        {
            // Clone references to `move` into upcoming event listener closure
            let app_state = self.state.clone();
            let canvas = canvas.clone();

            let mouse_move_listener = EventListener::new(&window, "mousemove", move |event| {
                let Some(event) = event.dyn_ref::<MouseEvent>() else {
                    return;
                };
                app_state.borrow_mut().handle_mouse_move(
                    &canvas,
                    event.client_x() as f64,
                    event.client_y() as f64,
                );
            });
            self.listeners.borrow_mut().push(mouse_move_listener);
        }

        // Mouse down
        {
            let app_state = self.state.clone();
            let frontend_callback = self.frontend_callback.clone();
            let canvas_target = canvas.clone();
            let canvas = canvas.clone();

            let mouse_move_listener =
                EventListener::new(&canvas_target, "mousedown", move |event| {
                    let Some(event) = event.dyn_ref::<MouseEvent>() else {
                        return;
                    };
                    let layer_id = app_state.borrow_mut().handle_mouse_down(
                        &canvas,
                        event.client_x() as f64,
                        event.client_y() as f64,
                    );

                    if let Some(layer_id) = layer_id {
                        if let Ok(callback) = frontend_callback.try_borrow_mut().as_deref_mut() {
                            if let Some(cb) = callback {
                                let key = layer_id.data().as_ffi();
                                let event = DesignEvent::LayerSelected { layer_id: key };
                                if let Ok(js_event) = serde_wasm_bindgen::to_value(&event) {
                                    cb.call1(&JsValue::NULL, &js_event);
                                }
                            }
                        }
                    };
                });
            self.listeners.borrow_mut().push(mouse_move_listener);
        }

        // Mouse up
        {
            let app_state = self.state.clone();

            let mouse_move_listener = EventListener::new(&window, "mouseup", move |_| {
                app_state.borrow_mut().handle_mouse_up();
            });
            self.listeners.borrow_mut().push(mouse_move_listener);
        }

        // Resize
        {
            let app_state = self.state.clone();
            let renderer = self.renderer.clone();
            let canvas_target = canvas.clone();

            let callback =
                Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
                    let Some(entry) = entries.get(0).dyn_into::<ResizeObserverEntry>().ok() else {
                        return;
                    };

                    // CSS pixel size of the element
                    let rect = entry.content_rect();
                    let dpr = web_sys::window()
                        .map(|w| w.device_pixel_ratio())
                        .unwrap_or(1.0);

                    let width = (rect.width() * dpr).round().max(1.0) as u32;
                    let height = (rect.height() * dpr).round().max(1.0) as u32;

                    // Keep the canvas backing store in sync with its display size
                    canvas_target.set_width(width);
                    canvas_target.set_height(height);

                    app_state
                        .borrow_mut()
                        .set_width_and_height(width as u16, height as u16);

                    if let Some(renderer) = renderer.borrow_mut().as_mut() {
                        renderer.resize(width, height);
                    }
                });

            let observer = ResizeObserver::new(callback.as_ref().unchecked_ref())
                .expect("failed to create ResizeObserver");
            observer.observe(&canvas);

            // Keep both alive. If you drop the closure, the callback becomes invalid.
            // Store them on `self` (or `.forget()` the closure if it lives for the app's lifetime).
            *self.resize_observer.borrow_mut() = Some((observer, callback));
        }

        Ok(())
    }

    // @TODO: Combine into a "stop" function for whole renderer
    #[wasm_bindgen]
    pub fn stop_event_listeners(&self) {
        if let Ok(mut listeners) = self.listeners.try_borrow_mut() {
            listeners.clear();
        }
    }
}
