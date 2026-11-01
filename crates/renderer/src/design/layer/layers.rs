use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use wasm_bindgen::{JsError, JsValue};

use crate::design::layer::{
    layer::{DraftLayer, Layer, LayerListResponse},
    layer_utils::convert_u64_to_default_key,
};

#[derive(Serialize, Clone)]
pub struct Layers {
    layers: SlotMap<DefaultKey, Layer>,
    selected_layer: DefaultKey,
}

impl Layers {
    pub fn new() -> Self {
        Self {
            layers: SlotMap::new(),
            selected_layer: Default::default(),
        }
    }

    pub fn add_layer(&mut self, layer: DraftLayer) {
        self.layers.insert(layer.into());
    }

    pub fn get_layer_list(&self) -> Option<JsValue> {
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
        let slot_map_key = convert_u64_to_default_key(layer_key);
        self.selected_layer = slot_map_key;
    }

    /// Provides a list of layers filtered by parent ID for backend.
    /// For top level layers, provide `None` as `parent` parameter.
    pub fn get_layers_by_parent(&self, parent: Option<DefaultKey>) -> Vec<(DefaultKey, &Layer)> {
        let layer_batch = self
            .layers
            .iter()
            .filter(|(_, layer)| layer.parent == parent)
            .collect();
        layer_batch
    }

    /// Provides a list of layers filtered by parent ID for frontend.
    /// Same as `get_layers_by_parent` - but this result is WASM JSON payload,
    /// and expects u64 parent (which is how frontend stores it).
    pub fn fetch_layers_by_parent(&self, parent: Option<u64>) -> Result<JsValue, JsError> {
        let parent_key = parent.map(|raw_key| convert_u64_to_default_key(raw_key));
        let layers = self.get_layers_by_parent(parent_key);

        let layer_response: Vec<LayerListResponse> = layers
            .iter()
            .map(|(k, v)| LayerListResponse {
                id: k.data().as_ffi(),
                name: v.name.clone(),
                visible: v.visible,
            })
            .collect();
        serde_wasm_bindgen::to_value(&layer_response).map_err(|err| JsError::new(&err.to_string()))
    }
}
