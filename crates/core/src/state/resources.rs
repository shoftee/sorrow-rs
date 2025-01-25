use std::sync::LazyLock;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use super::{recipes::CraftingRecipeKind, StateTable};

crate::state_key! {
    pub enum ResourceKind {
        Catnip,
        Wood,
    }
}

pub static CRAFTED_RESOURCES: LazyLock<AHashMap<ResourceKind, CraftingRecipeKind>> =
    LazyLock::new(|| {
        [(ResourceKind::Wood, CraftingRecipeKind::RefineCatnip)]
            .into_iter()
            .collect()
    });

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceTransport {
    pub amounts: StateTable<ResourceKind, f64>,
    pub deltas: StateTable<ResourceKind, f64>,
}
