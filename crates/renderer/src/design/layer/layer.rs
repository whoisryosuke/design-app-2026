use std::collections::BTreeMap;

use fractional_index::FractionalIndex;
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, KeyData};
use ts_rs::TS;

use crate::design::layer::layer_utils::convert_u64_to_default_key;

/// Defaults to (s)RGBA
#[derive(Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct LayerColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl LayerColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn get_components(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn update(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_red(r);
        self.set_green(g);
        self.set_blue(b);
        self.set_alpha(a);
    }

    pub fn set_red(&mut self, r: f32) {
        self.r = r.clamp(0.0, 1.0);
    }

    pub fn set_green(&mut self, g: f32) {
        self.g = g.clamp(0.0, 1.0);
    }

    pub fn set_blue(&mut self, b: f32) {
        self.b = b.clamp(0.0, 1.0);
    }

    pub fn set_alpha(&mut self, a: f32) {
        self.a = a.clamp(0.0, 1.0);
    }
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum FrameLayout {
    Flex,
    Grid,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum LayerKind {
    Group,
    Frame { layout: FrameLayout },
    Rect,
    Text(String),
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct UpdateLayer {
    pub name: Option<String>,
    pub kind: Option<LayerKind>,
    pub position_x: Option<f64>,
    pub position_y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub visible: Option<bool>,
    pub opacity: Option<f32>,
    pub color: Option<LayerColor>,
    pub parent: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DraftLayerItem {
    /// Order of nested layer.
    /// Uses fractional indexing in string form.
    order: String,
    /// ID of nested layer.
    /// We use u64 here and convert to SlotMap's DefaultKey later.
    key: u64,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DraftLayer {
    pub name: Option<String>,
    pub kind: LayerKind,
    pub position: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub visible: bool,
    pub opacity: f32,
    pub color: LayerColor,

    /// ID of parent layer.
    /// We use u64 here and convert to SlotMap's DefaultKey later.
    pub parent: Option<u64>,
    pub items: Vec<DraftLayerItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    pub name: String,
    pub kind: LayerKind,
    pub position: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub visible: bool,
    pub opacity: f32,
    pub color: LayerColor,

    pub parent: Option<DefaultKey>,
    pub items: BTreeMap<FractionalIndex, DefaultKey>,
}

impl From<DraftLayer> for Layer {
    fn from(draft_layer: DraftLayer) -> Self {
        // Convert u64 to DefaultKey since this is an ID for SlotMap
        let parent = draft_layer
            .parent
            .map(|raw_parent_key| convert_u64_to_default_key(raw_parent_key));

        // Convert Vec to BTree - along with fractional index and key inside.
        let items: BTreeMap<FractionalIndex, DefaultKey> = draft_layer
            .items
            .into_iter()
            .map(|item| {
                let order =
                    FractionalIndex::from_string(&item.order).unwrap_or(FractionalIndex::default());
                // Same process as parent above
                let item_key = convert_u64_to_default_key(item.key);
                (order, item_key)
            })
            .collect();

        Layer {
            name: draft_layer.name.unwrap_or("New Layer".to_string()),
            kind: draft_layer.kind,
            position: draft_layer.position,
            width: draft_layer.width,
            height: draft_layer.height,
            visible: draft_layer.visible,
            opacity: draft_layer.opacity,
            color: draft_layer.color,
            parent,
            items,
        }
    }
}

#[derive(Serialize)]
pub struct LayerListResponse {
    pub id: u64,
    pub name: String,
    pub visible: bool,
}
