use serde::{Deserialize, Serialize};

use crate::design::layer::layer::Layer;

#[derive(Serialize, Deserialize)]
pub enum DesignCommand {
    AddLayer { layer: Layer },
    SelectLayer { layer_id: u64 },
}
