use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use vello_gpu::{
    Scene,
    color::palette::css,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
    peniko::Color,
};
use wasm_bindgen::{JsError, JsValue};
use web_sys::{DomRect, HtmlCanvasElement};

use crate::design::layer::{
    layer::{DraftLayer, Layer},
    layers::Layers,
    layers_service::LayersService,
};

#[derive(Default, Serialize, Clone)]
pub struct MouseState {
    pub clicked: bool,
    pub last_position: (f64, f64),
}

#[derive(Serialize, Clone)]
pub struct AppState {
    pub layers_service: LayersService,
    width: u16,
    height: u16,
    mouse_state: MouseState,
}

impl AppState {
    pub fn new(width: Option<u16>, height: Option<u16>) -> Self {
        let layers_service = LayersService::new();
        Self {
            layers_service,
            width: width.unwrap_or(0),
            height: height.unwrap_or(0),
            mouse_state: MouseState::default(),
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
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
                x0: layer.position.0,
                y0: layer.position.1,
                x1: layer.position.0 + layer.width,
                y1: layer.position.1 + layer.height,
            });
        }

        // Draw selection
        if let Some(selected_layer) = self.layers_service.layers.get_selected_layer() {
            scene.set_stroke(Stroke::new(3.0));
            scene.set_paint(css::BLUE);
            scene.stroke_rect(&Rect {
                x0: selected_layer.position.0,
                y0: selected_layer.position.1,
                x1: selected_layer.position.0 + selected_layer.width,
                y1: selected_layer.position.1 + selected_layer.height,
            });
        }

        scene
    }

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene(timestamp)
    }

    pub fn handle_mouse_down(&mut self, canvas: &HtmlCanvasElement, x: f64, y: f64) {
        // Update mouse state
        self.mouse_state.clicked = true;

        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);
        self.mouse_state.last_position = (x, y);

        // Check if any layers can be selected
        self.find_layer_by_position(x, y);
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_state.clicked = false;
        self.mouse_state.last_position = (0.0, 0.0);
    }

    pub fn handle_mouse_move(&mut self, canvas: &HtmlCanvasElement, x: f64, y: f64) {
        let (x, y) = convert_mouse_to_canvas_position(&canvas, x, y);
        self.mouse_state.last_position = (x, y);
    }

    fn find_layer_by_position(&mut self, x: f64, y: f64) {
        if let Some((key, _)) = self.layers_service.layers.find_layer_by_position(x, y) {
            self.layers_service.layers.select_layer(key);
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
