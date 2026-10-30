use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use wasm_bindgen::JsValue;

use crate::design::layer::layer::{Layer, LayerListResponse};

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
}
