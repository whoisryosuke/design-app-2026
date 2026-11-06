use std::{cell::RefCell, rc::Rc};

use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use vello_gpu::{
    Scene,
    color::palette::css,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
    peniko::Color,
};
use wasm_bindgen::{JsError, JsValue};
use web_sys::{DomRect, HtmlCanvasElement, js_sys::Function};

use crate::{
    command_queue::design_event::DesignEvent,
    design::layer::{
        layer::{DraftLayer, Layer},
        layers::Layers,
        layers_service::LayersService,
    },
};

#[derive(Default, Serialize, Clone)]
pub enum InteractionMode {
    #[default]
    Idle,
    Selecting,
    Dragging,
}

#[derive(Default, Serialize, Clone)]
pub struct MouseState {
    pub clicked: bool,
    pub last_position: (f64, f64),
    pub interaction_mode: InteractionMode,
}

#[derive(Serialize, Clone)]
pub struct AppState {
    pub layers_service: LayersService,
    width: u16,
    height: u16,
    /// Resolution of user's device ("device pixel ratio")
    dpr: f64,
    mouse_state: MouseState,

    // Communication channel to frontend (also here to drop)
    #[serde(skip)]
    frontend_callback: Rc<RefCell<Option<Function>>>,
}

impl AppState {
    pub fn new(width: Option<u16>, height: Option<u16>) -> Self {
        let layers_service = LayersService::new();
        Self {
            layers_service,
            width: width.unwrap_or(0),
            height: height.unwrap_or(0),
            dpr: 1.0,
            mouse_state: MouseState::default(),
            frontend_callback: Rc::new(RefCell::new(None)),
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn set_resolution(&mut self, resolution: f64) {
        self.dpr = resolution;
    }

    /// Build a simple scene: background, a circle, and an outlined rounded rect.
    pub fn build_scene(&self, timestamp: f64) -> Scene {
        let mut scene = Scene::new(self.width, self.height);

        // Background
        scene.set_paint(css::DIM_GRAY);
        scene.fill_rect(&Rect::new(0.0, 0.0, self.width as f64, self.height as f64));

        // Filled circle
        // let distance = 40.0;
        // let animation_offset = (timestamp / 420.0).sin() * distance;
        // scene.set_paint(css::ORANGE);
        // scene.fill_path(&Circle::new((400.0 + animation_offset, 300.0), 120.0).to_path(0.1));

        // // Stroked rounded rectangle
        // scene.set_transform(Affine::IDENTITY);
        // scene.set_stroke(Stroke::new(8.0));
        // scene.set_paint(css::WHITE);
        // scene.stroke_path(
        //     &RoundedRect::new(
        //         200.0 + animation_offset,
        //         120.0 + animation_offset,
        //         600.0 + animation_offset,
        //         480.0 + animation_offset,
        //         24.0,
        //     )
        //     .to_path(0.1),
        // );

        let layers = self.layers_service.layers.get_layers_by_parent(None);

        for (_, layer) in layers {
            scene.set_paint(Color::new(layer.color.get_components()));
            scene.fill_rect(&Rect {
                x0: layer.position.0 * self.dpr,
                y0: layer.position.1 * self.dpr,
                x1: (layer.position.0 + layer.width) * self.dpr,
                y1: (layer.position.1 + layer.height) * self.dpr,
            });
        }

        // Draw selection
        if let Some(selected_layer) = self.layers_service.layers.get_selected_layer() {
            scene.set_stroke(Stroke::new(3.0));
            scene.set_paint(css::BLUE);
            scene.stroke_rect(&Rect {
                x0: selected_layer.position.0 * self.dpr,
                y0: selected_layer.position.1 * self.dpr,
                x1: (selected_layer.position.0 + selected_layer.width) * self.dpr,
                y1: (selected_layer.position.1 + selected_layer.height) * self.dpr,
            });
        }

        scene
    }

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene(timestamp)
    }

    /// Handle mouse down (aka click) on canvas.
    /// Updates local mouse position state, but also handles logic like
    /// selecting a layer or starting a drag sequence.
    pub fn handle_mouse_down(
        &mut self,
        canvas: &HtmlCanvasElement,
        x: f64,
        y: f64,
    ) -> Option<DefaultKey> {
        // Update mouse state
        self.mouse_state.clicked = true;

        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);
        self.mouse_state.last_position = (x, y);

        // Check if any layers can be selected
        let layer_id_result = self.find_layer_by_position(x, y);

        // Selected a layer? Change interaction mode + emit event to UI
        match layer_id_result {
            Some(layer_id) => {
                // Let frontend know layer was selected
                let key = layer_id.data().as_ffi();
                let event = DesignEvent::LayerSelected { layer_id: key };
                self.emit(event);

                // Let system know new interaction mode
                self.mouse_state.interaction_mode = InteractionMode::Selecting;
            }
            None => self.mouse_state.interaction_mode = InteractionMode::Idle,
        }

        layer_id_result
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_state.clicked = false;
        self.mouse_state.last_position = (0.0, 0.0);
        self.mouse_state.interaction_mode = InteractionMode::Idle;
    }

    /// Handle mouse move on canvas.
    /// Also handles dragging of layers based on `InteractionMode`.
    pub fn handle_mouse_move(&mut self, canvas: &HtmlCanvasElement, x: f64, y: f64) {
        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);

        // Handle "dragging" functionality
        match self.mouse_state.interaction_mode {
            // Draw selection box
            InteractionMode::Idle => {}
            // Enable dragging/movement of layer
            InteractionMode::Selecting => {
                self.mouse_state.interaction_mode = InteractionMode::Dragging;
            }
            // Move layer/object
            InteractionMode::Dragging => {
                // Get distance of drag (aka "delta") and remove resolution (aka scale to CSS-space)
                let delta_x = (x - self.mouse_state.last_position.0) / self.dpr;
                let delta_y = (y - self.mouse_state.last_position.1) / self.dpr;
                let movement_result = self.layers_service.drag_selected_layer(delta_x, delta_y);

                // Update UI with new position (useful for editor sidebar)
                // @TODO: Can we throttle this? Either here or in frontend.
                if let Some(data) = movement_result {
                    let event = DesignEvent::LayerMoved {
                        layer_id: data.layer_id,
                        position: data.position,
                    };
                    self.emit(event);
                }
            }
        }

        // Update last position last (in case anything needs to use it for delta math)
        self.mouse_state.last_position = (x, y);
    }

    /// Finds layer using position data. Returns layer or `None`.
    /// Assumes the position data is scaled to resolution (like mouse position).
    /// Don't use this to measure a CSS-based coordinated to find a layer
    /// (e.g. finding a layer using it's own position -- that's scaled to CSS).
    fn find_layer_by_position(&mut self, x: f64, y: f64) -> Option<DefaultKey> {
        // We get mouse coordinates scaled to resolution
        // so we convert it to "CSS" coordinates (same as design data)
        let x = x / self.dpr;
        let y = y / self.dpr;
        if let Some((key, _)) = self.layers_service.layers.find_layer_by_position(x, y) {
            self.layers_service.layers.select_layer(key);
            return Some(key);
        }
        return None;
    }

    pub fn setup_communication_callback(&self, callback: Function) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = Some(callback);
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't set callback, memory is locked")),
        }
    }

    pub fn stop_communication_callback(&self) -> Result<JsValue, JsError> {
        match self.frontend_callback.try_borrow_mut() {
            Ok(mut frontend_callback) => {
                *frontend_callback = None;
                Ok(JsValue::from_bool(true))
            }
            Err(_) => Err(JsError::new("Can't remove callback, memory is locked")),
        }
    }

    /// Emit events (`DesignEvent`) to the UI.
    /// Keeps the UI in sync with changes in backend.
    fn emit(&self, event: DesignEvent) {
        if let Ok(callback) = self.frontend_callback.try_borrow_mut().as_deref_mut() {
            if let Some(cb) = callback {
                let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
                if let Ok(js_event) = event.serialize(&serializer) {
                    cb.call1(&JsValue::NULL, &js_event);
                }
            }
        }
    }
}

fn convert_mouse_to_canvas_position(
    canvas: &HtmlCanvasElement,
    mouse_x: f64,
    mouse_y: f64,
) -> (f64, f64) {
    let rect = canvas.get_bounding_client_rect();
    let width = rect.width().max(1.0);
    let height = rect.height().max(1.0);
    let x = ((mouse_x - rect.left()) / width).clamp(0.0, 1.0);
    let y = ((mouse_y - rect.top()) / height).clamp(0.0, 1.0);
    (x * canvas.width() as f64, y * canvas.height() as f64)
}
