use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::design::layer::layer::{DraftLayer, UpdateLayer};

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", content = "payload")]
pub enum DesignCommand {
    AddLayer { layer: DraftLayer },
    SelectLayer { layer_id: u64 },
    FetchLayersByParent { parent_id: Option<u64> },
    UpdateLayer { layer_id: u64, data: UpdateLayer },
}
