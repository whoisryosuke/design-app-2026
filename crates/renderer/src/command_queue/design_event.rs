use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Serialize, Clone, TS)]
#[ts(export)]
pub enum MouseCursors {
    #[default]
    Default,
    Pointer,
    Move,
    ResizeHorizontal,
    ResizeVertical,
    ResizeDiagonalNwSe,
    ResizeDiagonalNeSw,
    Rotate,
}

#[derive(Serialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum DesignEvent {
    LayerSelected { layer_id: u64 },
    LayerMoved { layer_id: u64, position: (f64, f64) },
    MouseCursorChanged { cursor: MouseCursors },
}
