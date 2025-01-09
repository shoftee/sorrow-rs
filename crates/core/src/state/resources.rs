use serde::{Deserialize, Serialize};

use super::StateTable;

crate::state_key! {
    pub enum Kind {
        Catnip,
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceState {
    pub amounts: StateTable<Kind, f64>,
    pub deltas: StateTable<Kind, f64>,
}
