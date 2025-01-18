use serde::{Deserialize, Serialize};

use crate::state_key;

use super::StateTable;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Crafting(Crafting),
    Building(super::buildings::Kind),
}

impl Kind {
    #[expect(clippy::type_complexity)]
    pub fn iter() -> std::iter::Chain<
        std::iter::Map<super::buildings::KindIter, impl FnMut(super::buildings::Kind) -> Kind>,
        std::iter::Map<CraftingIter, impl FnMut(Crafting) -> Kind>,
    > {
        Iterator::chain(
            super::buildings::Kind::iter().map(Kind::Building),
            Crafting::iter().map(Kind::Crafting),
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
    pub crafting: StateTable<Crafting, Fulfillment>,
    pub building: StateTable<super::buildings::Kind, Fulfillment>,
}
