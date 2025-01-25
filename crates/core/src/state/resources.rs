use std::sync::LazyLock;

use ahash::AHashMap;

use super::recipes::CraftingRecipeKind;

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
