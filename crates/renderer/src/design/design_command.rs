use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::design::layer::layer::DraftLayer;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DesignCommand {
    AddLayer { layer: DraftLayer },
    SelectLayer { layer_id: u64 },
    FetchLayersByParent { parent_id: Option<u64> },
}
