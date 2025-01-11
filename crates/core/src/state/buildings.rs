use serde::{Deserialize, Serialize};

use super::StateTable;

crate::state_key! {
    pub enum Kind {
        CatnipField,
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildingState {
    pub levels: StateTable<Kind, u32>,
}
