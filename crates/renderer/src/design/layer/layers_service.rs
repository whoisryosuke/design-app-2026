use crate::design::layer::{
    layer::{DraftLayer, LayerListResponse},
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

    /// Provides a list of layers filtered by parent ID for frontend.
    /// Same as `layers.get_layers_by_parent` - but this result is WASM JSON payload,
    /// and expects u64 parent (which is how frontend stores it).
    pub fn fetch_layers_by_parent(&self, parent_id: Option<u64>) -> Result<JsValue, JsError> {
        let parent_key = parent_id.map(|raw_key| convert_u64_to_default_key(raw_key));
        let layers = self.layers.get_layers_by_parent(parent_key);

        let layer_response = Layers::create_layers_response(layers);

        serde_wasm_bindgen::to_value(&layer_response).map_err(|err| JsError::new(&err.to_string()))
    }

    pub fn select_layer(&mut self, layer_key: u64) {
        self.layers.select_layer(layer_key);
    }
}
