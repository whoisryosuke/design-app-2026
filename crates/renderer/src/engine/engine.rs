use crate::{
    app_state::AppState,
    command_queue::{command_queue::CommandQueue, design_command::DesignCommand},
    renderer::design_renderer::DesignRenderer,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct DesignEngine {
    state: Rc<RefCell<AppState>>,
    /// The WebGPU/Vello renderer for design documents.
    /// We use `RefCell` here to prevent issues with async fns.
    renderer: RefCell<Option<DesignRenderer>>,
    queue: Rc<RefCell<CommandQueue>>,
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

            // Mutable queue
            // These commands are queued and run before scene is compiled and rendered.
            _ => {
                self.queue.borrow_mut().add(cmd, false);
                Ok(JsValue::from_bool(true))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn create_render(&self, canvas_id: &str) -> Result<JsValue, JsError> {
        let renderer = DesignRenderer::create(canvas_id)
            .await
            .map_err(|error_msg| JsError::new(&error_msg.to_string()))?;

        *self.renderer.borrow_mut() = Some(renderer);
        Ok(JsValue::from_bool(true))
    }

    #[wasm_bindgen]
    pub fn start_renderer(&mut self) -> Result<JsValue, JsError> {
        let mut renderer_result = self.renderer.borrow_mut();
        match renderer_result.as_mut() {
            Some(renderer) => {
                renderer.run(self.state.clone(), self.queue.clone());
                Ok(JsValue::from_bool(true))
            }
            None => Err(JsError::new(
                &"No renderer found, create one first".to_string(),
            )),
        }
    }

    #[wasm_bindgen]
    pub fn stop_renderer(&mut self) -> Result<JsValue, JsError> {
        let mut renderer_result = self.renderer.borrow_mut();
        match renderer_result.as_mut() {
            Some(renderer) => {
                renderer.stop();
                Ok(JsValue::from_bool(true))
            }
            None => Err(JsError::new(
                &"No renderer found, create one first".to_string(),
            )),
        }
    }
}
