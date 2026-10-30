use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use vello_gpu::{
    Scene,
    color::palette::css,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
};
use wasm_bindgen::JsValue;

use crate::design::layer::{Layer, LayerListResponse};

#[derive(Serialize, Clone)]
pub struct AppState {
    layers: SlotMap<DefaultKey, Layer>,
    selected_layer: DefaultKey,
    width: u16,
    height: u16,
}

impl AppState {
    pub fn new(width: u16, height: u16) -> Self {
        let layers = SlotMap::new();
        Self {
            layers,
            selected_layer: Default::default(),
            width,
            height,
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.insert(layer);
    }

    pub fn get_layers(&self) -> Option<JsValue> {
        let list: Vec<LayerListResponse> = self
            .layers
            .iter()
            .map(|(k, v)| LayerListResponse {
                id: k.data().as_ffi(),
                name: v.name.clone(),
                visible: v.visible,
            })
            .collect();
        serde_wasm_bindgen::to_value(&list).ok()
    }

    pub fn select_layer(&mut self, layer_key: u64) {
        let key_data = KeyData::from_ffi(layer_key);
        let slot_map_key: DefaultKey = key_data.into();

        self.selected_layer = slot_map_key;
    }

    /// Build a simple scene: background, a circle, and an outlined rounded rect.
    pub fn build_scene(&self, timestamp: f64) -> Scene {
        let mut scene = Scene::new(self.width, self.height);

        // Background
        scene.set_paint(css::BLUE);
        scene.fill_rect(&Rect::new(0.0, 0.0, self.width as f64, self.height as f64));

        // Filled circle
        let distance = 40.0;
        let animation_offset = (timestamp / 420.0).sin() * distance;
        scene.set_paint(css::ORANGE);
        scene.fill_path(&Circle::new((400.0 + animation_offset, 300.0), 120.0).to_path(0.1));

        // Stroked rounded rectangle
        scene.set_transform(Affine::IDENTITY);
        scene.set_stroke(Stroke::new(8.0));
        scene.set_paint(css::WHITE);
        scene.stroke_path(
            &RoundedRect::new(
                200.0 + animation_offset,
                120.0 + animation_offset,
                600.0 + animation_offset,
                480.0 + animation_offset,
                24.0,
            )
            .to_path(0.1),
        );

        scene
    }

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene(timestamp)
    }
}
