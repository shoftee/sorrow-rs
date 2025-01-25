use serde::{Deserialize, Serialize};

use super::StateTable;

crate::state_key! {
    pub enum BuildingKind {
        CatnipField,
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildingTransport {
    pub levels: StateTable<BuildingKind, u32>,
}
