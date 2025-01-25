use serde::{Deserialize, Serialize};

use super::StateTable;

crate::state_key! {
    pub enum ResourceKind {
        Catnip,
        Wood,
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceTransport {
    pub amounts: StateTable<ResourceKind, f64>,
    pub deltas: StateTable<ResourceKind, f64>,
}
