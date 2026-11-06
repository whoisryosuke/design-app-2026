use serde::Serialize;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use ts_rs::TS;
use vello_gpu::kurbo::{Point, Rect};

use crate::design::layer::{
    layer::{DraftLayer, Layer, LayerListResponse},
    layer_utils::{convert_default_key_to_u64, convert_u64_to_default_key},
};

#[derive(Serialize, Clone, TS)]
#[ts(export)]
pub struct DragLayerUpdate {
    pub layer_id: u64,
    pub position: (f64, f64),
}

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

    pub fn get_mutable_layer(&mut self, id: DefaultKey) -> Option<&mut Layer> {
        self.layers.get_mut(id)
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

    pub fn select_layer(&mut self, layer_key: DefaultKey) {
        self.selected_layer = layer_key;
    }

    pub fn get_selected_layer_id(&self) -> DefaultKey {
        self.selected_layer
    }

    pub fn get_selected_layer(&self) -> Option<&Layer> {
        self.layers.get(self.selected_layer)
    }

    pub fn get_mutable_selected_layer(&mut self) -> Option<&mut Layer> {
        self.layers.get_mut(self.selected_layer)
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

    pub fn find_layer_by_position(&self, x: f64, y: f64) -> Option<(DefaultKey, Layer)> {
        self.layers
            .iter()
            .find_map(|(key, layer)| self.hit_test(&key, &layer, x, y))

        // .values()
        // .find_map(|layer| self.hit_test(&layer, x, y))
    }

    fn hit_test(
        &self,
        key: &DefaultKey,
        layer: &Layer,
        x: f64,
        y: f64,
    ) -> Option<(DefaultKey, Layer)> {
        let local = Point::new(x, y);

        // Children are drawn on top of their parent, so check them first
        // if let Some(hit) = self.hit_test_children(key, local) {
        //     return Some(hit);
        // }

        let bounds = Rect::new(
            layer.position.0,
            layer.position.1,
            layer.position.0 + layer.width,
            layer.position.1 + layer.height,
        );
        bounds
            .contains(local)
            .then_some((key.clone(), layer.clone()))
    }

    pub fn drag_selected_layer(&mut self, x: f64, y: f64) -> Option<DragLayerUpdate> {
        let layer = self
            .layers
            .iter_mut()
            .find(|(layer_id, _)| *layer_id == self.selected_layer);

        if let Some((key, layer_data)) = layer {
            let new_x = layer_data.position.0 + x;
            let new_y = layer_data.position.1 + y;
            layer_data.position.0 = new_x;
            layer_data.position.1 = new_y;

            return Some(DragLayerUpdate {
                layer_id: convert_default_key_to_u64(key),
                position: (new_x, new_y),
            });
        }
        None
    }
}
