use std::sync::LazyLock;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use super::StateTable;

crate::state_key! {
    pub enum BuildingKind {
        CatnipField,
    }
}

pub static BUILDING_PRICE_RATIOS: LazyLock<AHashMap<BuildingKind, f64>> =
    LazyLock::new(|| [(BuildingKind::CatnipField, 1.12)].into_iter().collect());

pub static BUILDING_UNLOCK_RATIOS: LazyLock<AHashMap<BuildingKind, f64>> =
    LazyLock::new(|| [(BuildingKind::CatnipField, 0.3)].into_iter().collect());

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildingTransport {
    pub levels: StateTable<BuildingKind, u32>,
}
