use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{recipes, resources, KeyIter, StateTable};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeId {
    Resources(ResourceNodeId),
    Bonfire(BonfireNodeId),
}

impl KeyIter for NodeId {
    type Item = NodeId;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        Iterator::chain(
            <ResourceNodeId as KeyIter>::key_iter().map(NodeId::Resources),
            <BonfireNodeId as KeyIter>::key_iter().map(NodeId::Bonfire),
        )
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

impl From<resources::Kind> for NodeId {
    fn from(value: resources::Kind) -> Self {
        match value {
            resources::Kind::Catnip => NodeId::Resources(ResourceNodeId::Catnip),
            resources::Kind::Wood => NodeId::Resources(ResourceNodeId::Wood),
        }
    }
}

state_key!(
    pub enum ResourceNodeId {
        Catnip,
        Wood,
    }
);

impl From<ResourceNodeId> for super::resources::Kind {
    fn from(value: ResourceNodeId) -> Self {
        match value {
            ResourceNodeId::Catnip => super::resources::Kind::Catnip,
            ResourceNodeId::Wood => super::resources::Kind::Wood,
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
