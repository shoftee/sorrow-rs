mod table;

pub use table::*;

use serde::{Deserialize, Serialize};

use crate::state::{
    buildings::BuildingKind,
    calendar::SeasonKind,
    recipes::{CraftingRecipeKind, FulfillmentState, RecipeKind},
    resources::ResourceKind,
    time::RunningState,
    ui::NodeId,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    QueueWorkOrder(WorkOrderKind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkOrderKind {
    Craft(CraftingRecipeKind),
    Construct(BuildingKind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeControl {
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EngineMessage {
    Loaded,
    Updated(Vec<EngineUpdate>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EngineUpdate {
    CalendarChanged(CalendarTransport),
    BuildingsChanged(BuildingTransport),
    FulfillmentsChanged(FulfillmentTransport),
    ResourcesChanged(ResourceTransport),
    TimeChanged(TimeTransport),
    VisibilityChanged(VisibilityTransport),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BuildingTransport {
    pub levels: StateTable<BuildingKind, u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FulfillmentTransport {
    pub fulfillments: StateTable<RecipeKind, FulfillmentState>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CalendarTransport {
    pub day: Option<i16>,
    pub season: Option<SeasonKind>,
    pub year: Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ResourceTransport {
    pub amounts: StateTable<ResourceKind, f64>,
    pub deltas: StateTable<ResourceKind, f64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TimeTransport {
    pub running_state: Option<RunningState>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VisibilityTransport {
    pub nodes: StateTable<NodeId, bool>,
}
