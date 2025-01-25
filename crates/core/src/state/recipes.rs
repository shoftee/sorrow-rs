use std::sync::LazyLock;

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{buildings::BuildingKind, resources::ResourceKind, KeyIter, StateTable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecipeKind {
    Crafting(CraftingRecipeKind),
    Building(BuildingKind),
}

impl KeyIter for RecipeKind {
    type Item = RecipeKind;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        Iterator::chain(
            BuildingKind::key_iter().map(RecipeKind::Building),
            CraftingRecipeKind::key_iter().map(RecipeKind::Crafting),
        )
    }
}

#[derive(Debug)]
pub struct ResourceAmount(pub ResourceKind, pub f64);

state_key!(
    pub enum CraftingRecipeKind {
        GatherCatnip,
        RefineCatnip,
    }
);

pub static RECIPE_INGREDIENTS: LazyLock<AHashMap<RecipeKind, Vec<ResourceAmount>>> =
    LazyLock::new(|| {
        [
            (
                RecipeKind::Crafting(CraftingRecipeKind::GatherCatnip),
                vec![],
            ),
            (
                RecipeKind::Crafting(CraftingRecipeKind::RefineCatnip),
                vec![ResourceAmount(ResourceKind::Catnip, 100.0)],
            ),
            (
                RecipeKind::Building(BuildingKind::CatnipField),
                vec![ResourceAmount(ResourceKind::Catnip, 10.0)],
            ),
        ]
        .into_iter()
        .collect()
    });

pub static RECIPE_CRAFTED_RESOURCES: LazyLock<AHashMap<CraftingRecipeKind, ResourceAmount>> =
    LazyLock::new(|| {
        [
            (
                CraftingRecipeKind::GatherCatnip,
                ResourceAmount(ResourceKind::Catnip, 1.0),
            ),
            (
                CraftingRecipeKind::RefineCatnip,
                ResourceAmount(ResourceKind::Wood, 1.0),
            ),
        ]
        .into_iter()
        .collect()
    });

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FulfillmentState {
    #[default]
    Unfulfilled,
    Fulfilled,
    Capped,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FulfillmentTransport {
    pub fulfillments: StateTable<RecipeKind, FulfillmentState>,
}
