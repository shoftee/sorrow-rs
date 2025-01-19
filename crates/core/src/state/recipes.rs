use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{KeyIter, StateTable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Crafting(Crafting),
    Building(super::buildings::Kind),
}

impl KeyIter for Kind {
    type Item = Kind;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        Iterator::chain(
            super::buildings::Kind::key_iter().map(Kind::Building),
            Crafting::key_iter().map(Kind::Crafting),
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
    pub fulfillments: StateTable<Kind, Fulfillment>,
}
