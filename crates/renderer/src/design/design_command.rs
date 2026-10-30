use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DesignCommand {
    AddLayer,
    SelectLayer { layer_id: u64 },
}
