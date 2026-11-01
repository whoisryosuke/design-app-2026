use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::{
    app_state::AppState, design::design_command::DesignCommand,
    renderer::renderer_context::RendererContext, request_animation_frame,
};

#[wasm_bindgen]
pub struct DesignRenderer {
    state: Rc<RefCell<AppState>>,
    canvas: Option<HtmlCanvasElement>,
    renderer_context: Rc<RefCell<RendererContext>>,
}

#[wasm_bindgen]
impl DesignRenderer {
    /// Create a new instance of DesignRenderer. Initializes the app state and renderer backend.
    /// We do this in a `create()` vs `new()` because JS doesn't support async constructor.
    #[wasm_bindgen]
    pub async fn create(canvas_id: &str) -> Result<Self, JsValue> {
        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&canvas_id))
            .ok_or_else(|| JsValue::from_str(&format!("No element with id `{canvas_id}`")))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a <canvas>"))?;

        let state = AppState::new(canvas.width() as u16, canvas.height() as u16);
        let renderer_context = RendererContext::new(canvas.clone()).await;

        Ok(DesignRenderer {
            state: Rc::new(RefCell::new(state)),
            canvas: Some(canvas),
            renderer_context: Rc::new(RefCell::new(renderer_context)),
        })
    }

    /// Handle commands from frontend (e.g. create layer)
    #[wasm_bindgen]
    pub fn dispatch(&mut self, cmd_js: JsValue) -> Result<JsValue, JsError> {
        // Convert JS object directly to Rust Enum
        let cmd = serde_wasm_bindgen::from_value::<DesignCommand>(cmd_js)
            .map_err(|_| JsError::new("Not a proper command. Maybe mispelled?"))?;
        match cmd {
            DesignCommand::AddLayer { layer } => {
                self.state.borrow_mut().add_layer(layer);
                Ok(JsValue::from_bool(true))
            }
            DesignCommand::SelectLayer { layer_id } => {
                self.state.borrow_mut().select_layer(layer_id);
                Ok(JsValue::from_bool(true))
            }
            DesignCommand::FetchLayersByParent { parent_id } => {
                let response = self.state.borrow_mut().fetch_layers_by_parent(parent_id);
                response
            }
        }
    }

    /// Manually render. Not used since we have RAF run render process.
    async fn render(&mut self) -> Result<(), JsValue> {
        let scene = self.state.borrow().build_scene(0.0);
        self.renderer_context.borrow_mut().render(&scene);

        Ok(())
    }

    /// Start the render loop using RAF.
    /// Builds the scene using AppState and then renders using backend.
    #[wasm_bindgen]
    pub async fn run(&self) {
        {
            // Create a placeholder for the RAF closure to loop again
            let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
            // Closure for the initial/first RAF closure
            let g = f.clone();

            // Clone pointers to state (via `Rc`) to move into closure
            let app_state = self.state.clone();
            let renderer = self.renderer_context.clone();

            // Define our animation function / closure
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
                // Update state with latest time and get latest scene data
                let scene = app_state.borrow_mut().frame(timestamp);
                // Render scene data
                renderer.borrow_mut().render(&scene);

                // Keep looping
                request_animation_frame(f.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut(f64)>));

            // Start animation using wasm_bindgen/browser's RAF
            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    }
}
