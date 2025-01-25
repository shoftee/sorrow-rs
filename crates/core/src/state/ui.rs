use serde::{Deserialize, Serialize};

use crate::state_key;

use super::{
    buildings::BuildingKind,
    recipes::{CraftingRecipeKind, RecipeKind},
    resources::ResourceKind,
    KeyIter, StateTable,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeId {
    Navigation(NavigationNodeId),
    Resources(ResourceNodeId),
    Bonfire(BonfireNodeId),
}

impl KeyIter for NodeId {
    type Item = NodeId;

    fn key_iter() -> impl Iterator<Item = Self::Item> {
        itertools::chain![
            <NavigationNodeId as KeyIter>::key_iter().map(NodeId::Navigation),
            <ResourceNodeId as KeyIter>::key_iter().map(NodeId::Resources),
            <BonfireNodeId as KeyIter>::key_iter().map(NodeId::Bonfire),
        ]
    }
}

impl From<RecipeKind> for NodeId {
    fn from(value: RecipeKind) -> Self {
        match value {
            RecipeKind::Crafting(crafting) => match crafting {
                CraftingRecipeKind::GatherCatnip => NodeId::Bonfire(BonfireNodeId::GatherCatnip),
                CraftingRecipeKind::RefineCatnip => NodeId::Bonfire(BonfireNodeId::RefineCatnip),
            },
            RecipeKind::Building(BuildingKind::CatnipField) => {
                NodeId::Bonfire(BonfireNodeId::CatnipField)
            }
        }
    }
}

impl From<ResourceKind> for NodeId {
    fn from(value: ResourceKind) -> Self {
        match value {
            ResourceKind::Catnip => NodeId::Resources(ResourceNodeId::Catnip),
            ResourceKind::Wood => NodeId::Resources(ResourceNodeId::Wood),
        }
    }
}

state_key!(
    pub enum ResourceNodeId {
        Catnip,
        Wood,
    }
);

impl From<ResourceNodeId> for ResourceKind {
    fn from(value: ResourceNodeId) -> Self {
        match value {
            ResourceNodeId::Catnip => ResourceKind::Catnip,
            ResourceNodeId::Wood => ResourceKind::Wood,
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

state_key!(
    pub enum NavigationNodeId {
        Bonfire,
    }
);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VisibilityTransport {
    pub nodes: StateTable<NodeId, bool>,
}
