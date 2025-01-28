mod table;
mod transport;

pub use table::*;
pub use transport::*;

use serde::{Deserialize, Serialize};

use crate::state::{buildings::BuildingKind, recipes::CraftingRecipeKind};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeControl {
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkOrderKind {
    Craft(CraftingRecipeKind),
    Construct(BuildingKind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    QueueWorkOrder(WorkOrderKind),
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
