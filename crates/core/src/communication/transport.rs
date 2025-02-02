use serde::{Deserialize, Serialize};

use crate::state::{
    buildings::BuildingKind,
    calendar::SeasonKind,
    recipes::{FulfillmentState, RecipeKind},
    resources::ResourceKind,
    time::RunningState,
    ui::NodeId,
};

use super::StateTable;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BuildingTransport {
    pub levels: StateTable<BuildingKind, u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FulfillmentTransport {
    pub fulfillments: StateTable<RecipeKind, FulfillmentState>,
    pub required_amounts: StateTable<(RecipeKind, ResourceKind), f64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CalendarTransport {
    pub day: Option<i16>,
    pub season: Option<SeasonKind>,
    pub year: Option<usize>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResourceTransport {
    pub amounts: StateTable<ResourceKind, f64>,
    pub deltas: StateTable<ResourceKind, f64>,
    pub capacities: StateTable<ResourceKind, Option<f64>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TimeTransport {
    pub running_state: Option<RunningState>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VisibilityTransport {
    pub nodes: StateTable<NodeId, bool>,
}
