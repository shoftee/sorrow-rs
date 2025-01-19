use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{recipes, KeyIter, StateTable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeId {
    Bonfire(BonfireNodeId),
}

impl KeyIter for NodeId {
    type Item = NodeId;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        <BonfireNodeId as KeyIter>::key_iter().map(NodeId::Bonfire)
    }
}

impl From<recipes::Kind> for NodeId {
    fn from(value: recipes::Kind) -> Self {
        match value {
            recipes::Kind::Crafting(crafting) => match crafting {
                recipes::Crafting::GatherCatnip => NodeId::Bonfire(BonfireNodeId::GatherCatnip),
                recipes::Crafting::RefineCatnip => NodeId::Bonfire(BonfireNodeId::RefineCatnip),
            },
            recipes::Kind::Building(super::buildings::Kind::CatnipField) => {
                NodeId::Bonfire(BonfireNodeId::CatnipField)
            }
        }
    }
}

state_key!(
    pub enum BonfireNodeId {
        GatherCatnip,
        RefineCatnip,
        CatnipField,
    }
);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VisibilityState {
    pub nodes: StateTable<NodeId, bool>,
}
