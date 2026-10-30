use std::collections::BTreeMap;

use fractional_index::FractionalIndex;
use serde::Serialize;
use slotmap::DefaultKey;

#[derive(Serialize, Clone)]
pub enum FrameLayout {
    Flex,
    Grid,
}

#[derive(Serialize, Clone)]
pub enum LayerKind {
    Group,
    Frame { layout: FrameLayout },
    Rect,
    Text(String),
}

#[derive(Serialize, Clone)]
pub struct Layer {
    pub name: String,
    pub kind: LayerKind,
    pub position: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub visible: bool,
    pub opacity: f32,
    pub parent: Option<DefaultKey>,
    pub items: BTreeMap<FractionalIndex, DefaultKey>,
}

#[derive(Serialize)]
pub struct LayerListResponse {
    pub id: u64,
    pub name: String,
    pub visible: bool,
}
