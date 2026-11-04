use crate::{
    app_state::AppState,
    command_queue::{command_queue::CommandQueue, design_command::DesignCommand},
    renderer::design_renderer::DesignRenderer,
};
use gloo_events::EventListener;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
struct DesignEngine {
    state: Rc<RefCell<AppState>>,
    /// The WebGPU/Vello renderer for design documents.
    /// We use `RefCell` here to prevent issues with async fns.
    renderer: RefCell<Option<DesignRenderer>>,
    queue: Rc<RefCell<CommandQueue>>,
    /// Numeric flag (aka "Monotonic token") used to discard stale renderer
    create_generation: RefCell<u64>,
    // Event listeners (here to drop later)
    listeners: RefCell<Vec<EventListener>>,
}

#[wasm_bindgen]
impl DesignEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        let state = AppState::new(None, None);
        let queue = CommandQueue::new();

        Ok(DesignEngine {
            state: Rc::new(RefCell::new(state)),
            renderer: RefCell::new(None),
            queue: Rc::new(RefCell::new(queue)),
            create_generation: RefCell::new(0),
            listeners: RefCell::new(Vec::new()),
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
            let canvas_target = canvas.clone();
            let canvas = canvas.clone();

            let mouse_move_listener =
                EventListener::new(&canvas_target, "mousedown", move |event| {
                    let Some(event) = event.dyn_ref::<MouseEvent>() else {
                        return;
                    };
                    app_state.borrow_mut().handle_mouse_down(
                        &canvas,
                        event.client_x() as f64,
                        event.client_y() as f64,
                    );
                });
            self.listeners.borrow_mut().push(mouse_move_listener);
        }

        // Mouse up
        {
            let app_state = self.state.clone();

            let mouse_move_listener = EventListener::new(&window, "mousedown", move |_| {
                app_state.borrow_mut().handle_mouse_up();
            });
            self.listeners.borrow_mut().push(mouse_move_listener);
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn stop_event_listeners(&self) {
        if let Ok(mut listeners) = self.listeners.try_borrow_mut() {
            listeners.clear();
        }
    }
}
