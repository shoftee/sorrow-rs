use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{buildings::BuildingKind, KeyIter, StateTable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecipeKind {
    Crafting(Crafting),
    Building(BuildingKind),
}

impl KeyIter for RecipeKind {
    type Item = RecipeKind;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        Iterator::chain(
            BuildingKind::key_iter().map(RecipeKind::Building),
            Crafting::key_iter().map(RecipeKind::Crafting),
        )
    }
}

state_key!(
    pub enum Crafting {
        GatherCatnip,
        RefineCatnip,
    }
);

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fulfillment {
    #[default]
    Unfulfilled,
    Fulfilled,
    Capped,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FulfillmentState {
    pub fulfillments: StateTable<RecipeKind, Fulfillment>,
}
