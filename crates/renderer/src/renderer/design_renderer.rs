use std::{cell::RefCell, rc::Rc};

use vello_gpu::Scene;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{
    app_state::AppState, command_queue::command_queue::CommandQueue,
    renderer::renderer_context::RendererContext, request_animation_frame,
};

pub struct DesignRenderer {
    canvas: Option<HtmlCanvasElement>,
    renderer_context: Rc<RefCell<RendererContext>>,
    loop_handle: Option<Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>>,
}

impl DesignRenderer {
    /// Create a new instance of DesignRenderer. Initializes the app state and renderer backend.
    /// We do this in a `create()` vs `new()` because JS doesn't support async constructor.
    pub async fn create(canvas_id: &str) -> Result<Self, String> {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&canvas_id))
            .ok_or_else(|| format!("No element with id `{canvas_id}`"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a <canvas>".to_string())?;

        let renderer_context = RendererContext::new(canvas.clone()).await;

        Ok(DesignRenderer {
            canvas: Some(canvas),
            renderer_context: Rc::new(RefCell::new(renderer_context)),
            loop_handle: None,
        })
    }

    pub fn get_canvas_size(&self) -> Result<(u32, u32), String> {
        let canvas = self
            .canvas
            .as_ref()
            .ok_or_else(|| "No canvas, recreate this struct".to_string())?;

        Ok((canvas.width(), canvas.height()))
    }

    pub fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        self.canvas.clone()
    }

    pub fn get_canvas_resolution(&self) -> f64 {
        web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.0)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer_context.borrow_mut().resize(width, height);
    }

    /// Manually render. Not used since we have RAF run render process.
    async fn render(&mut self, scene: &Scene) -> Result<(), JsValue> {
        self.renderer_context.borrow_mut().render(scene);

        Ok(())
    }

    /// Stops the render loop by dropping the handle.
    pub fn stop(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            *handle.borrow_mut() = None;
        }
    }

    /// Start the render loop using RAF.
    /// Builds the scene using AppState and then renders using backend.
    pub fn run(&mut self, app_state: Rc<RefCell<AppState>>, queue: Rc<RefCell<CommandQueue>>) {
        // Prevent multiple loops from running simultaneously
        if self.loop_handle.is_some() {
            return;
        }

        // Create a placeholder for the RAF closure to loop again
        let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
        // Closure for the initial/first RAF closure
        let g = f.clone();

        // Clone pointers to state (via `Rc`) to move into closure
        let app_state = app_state.clone();
        let queue = queue.clone();
        let renderer = self.renderer_context.clone();
        let f_for_closure = f.clone(); // Clone for the closure to use inside itself

        // Define our animation function / closure
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
            // Update state with latest time and get latest scene data
            // We do it inside a block here to ensure app_state's borrow is dropped earlier.
            let scene = {
                // Run queue with mutable commands and update state
                // Again, also block-encased to drop mutable state sooner.
                {
                    let mut state = app_state.borrow_mut();
                    queue.borrow_mut().drain(&mut state);
                }
                app_state.borrow().frame(timestamp)
            };

            // Render scene data
            renderer.borrow_mut().render(&scene);

            // Check if we stopped (by dropping the `loop_handle`)
            // or keep looping using RAF
            if let Some(ref closure) = *f_for_closure.borrow() {
                request_animation_frame(closure);
            }
        }) as Box<dyn FnMut(f64)>));

        // Start animation using wasm_bindgen/browser's RAF
        request_animation_frame(g.borrow().as_ref().unwrap());

        // Store the handle in the struct so it lives as long as the renderer
        self.loop_handle = Some(f);
    }
}
