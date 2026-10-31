use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use vello_gpu::{
    Scene,
    color::palette::css,
    kurbo::{Affine, Circle, Rect, RoundedRect, Shape, Stroke},
};
use wasm_bindgen::JsValue;

use crate::design::layer::{
    layer::{DraftLayer, Layer},
    layers::Layers,
};

#[derive(Serialize, Clone)]
pub struct AppState {
    layers: Layers,
    width: u16,
    height: u16,
}

impl AppState {
    pub fn new(width: u16, height: u16) -> Self {
        let layers = Layers::new();
        Self {
            layers,
            width,
            height,
        }
    }

    pub fn set_width_and_height(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn add_layer(&mut self, layer: DraftLayer) {
        self.layers.add_layer(layer);
    }

    pub fn get_layers(&self) -> Option<JsValue> {
        self.layers.get_layer_list()
    }

    pub fn select_layer(&mut self, layer_key: u64) {
        self.layers.select_layer(layer_key);
    }

    /// Build a simple scene: background, a circle, and an outlined rounded rect.
    pub fn build_scene(&self, timestamp: f64) -> Scene {
        let mut scene = Scene::new(self.width, self.height);

        // Background
        scene.set_paint(css::BLUE);
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

        let layers = self.layers.get_layers_by_parent(None);

        for (_, layer) in layers {
            scene.set_paint(css::WHITE);
            scene.fill_rect(&Rect {
                x0: layer.position.0,
                y0: layer.position.0,
                x1: layer.width,
                y1: layer.height,
            });
        }

        scene
    }

    pub fn frame(&self, timestamp: f64) -> Scene {
        self.build_scene(timestamp)
    }
}
