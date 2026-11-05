use crate::design::layer::{
    layer::{DraftLayer, LayerListResponse, UpdateLayer},
    layer_utils::convert_u64_to_default_key,
    layers::Layers,
};
use serde::Serialize;
use slotmap::Key;
use wasm_bindgen::{JsError, JsValue};

/// A wrapper around Layers that provides WASM-friendly methods.
/// To keep Layers "pure", we separate any WASM logic here,
/// like returning JSON responses for the frontend.
/// Need to use Layers directly for backend? Use the param.
#[derive(Serialize, Clone)]
pub struct LayersService {
    /// The current document's layers.
    pub layers: Layers,
}

impl LayersService {
    pub fn new() -> Self {
        let layers = Layers::new();
        Self { layers }
    }

    pub fn add_layer(&mut self, layer: DraftLayer) -> u64 {
        let key = self.layers.add_layer(layer);
        let raw_key = key.data().as_ffi();
        raw_key
    }

    pub fn fetch_layers(&self) -> Option<JsValue> {
        let list = self.layers.get_layer_list();
        serde_wasm_bindgen::to_value(&list).ok()
    }

    pub fn select_layer(&mut self, layer_key: u64) {
        let slot_map_key = convert_u64_to_default_key(layer_key);
        self.layers.select_layer(slot_map_key);
    }

    /// Provides a list of layers filtered by parent ID for frontend.
    /// Same as `layers.get_layers_by_parent` - but this result is WASM JSON payload,
    /// and expects u64 parent (which is how frontend stores it).
    pub fn fetch_layers_by_parent(&self, parent_id: Option<u64>) -> Result<JsValue, JsError> {
        let parent_key = parent_id.map(|raw_key| convert_u64_to_default_key(raw_key));
        let layers = self.layers.get_layers_by_parent(parent_key);

        let layer_response = Layers::create_layers_response(layers);

        serde_wasm_bindgen::to_value(&layer_response).map_err(|err| JsError::new(&err.to_string()))
    }

    pub fn update_layer(&mut self, layer_id: u64, data: UpdateLayer) {
        let layer_id = convert_u64_to_default_key(layer_id);

        if let Some(layer) = self.layers.get_mutable_layer(layer_id) {
            // Update each property that was provided
            if let Some(name) = data.name {
                layer.name = name;
            }
            if let Some(kind) = data.kind {
                layer.kind = kind;
            }
            if let Some(position_x) = data.position_x {
                layer.position.0 = position_x;
            }
            if let Some(position_y) = data.position_y {
                layer.position.1 = position_y;
            }
            if let Some(width) = data.width {
                layer.width = width;
            }
            if let Some(height) = data.height {
                layer.height = height;
            }
            if let Some(visible) = data.visible {
                layer.visible = visible;
            }
            if let Some(opacity) = data.opacity {
                layer.opacity = opacity;
            }
            if let Some(color) = data.color {
                layer.color = color;
            }
            if let Some(parent) = data.parent {
                layer.parent = Some(convert_u64_to_default_key(parent));
            }
        }
    }

    pub fn drag_selected_layer(&mut self, x: f64, y: f64) {
        self.layers.drag_selected_layer(x, y);
    }
}
