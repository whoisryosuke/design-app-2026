use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};

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

    pub fn add_layer(&mut self, layer: DraftLayer) -> DefaultKey {
        self.layers.insert(layer.into())
    }

    pub fn get_layer_list(&self) -> Vec<LayerListResponse> {
        let list: Vec<LayerListResponse> = self
            .layers
            .iter()
            .map(|(k, v)| LayerListResponse {
                id: k.data().as_ffi(),
                name: v.name.clone(),
                visible: v.visible,
            })
            .collect();
        list
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

    pub fn create_layers_response(layers: Vec<(DefaultKey, &Layer)>) -> Vec<LayerListResponse> {
        let layer_response: Vec<LayerListResponse> = layers
            .iter()
            .map(|(k, v)| LayerListResponse {
                id: k.data().as_ffi(),
                name: v.name.clone(),
                visible: v.visible,
            })
            .collect();
        layer_response
    }
}
