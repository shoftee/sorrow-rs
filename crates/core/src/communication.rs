use serde::{Deserialize, Serialize};

use crate::state::{buildings::BuildingKind, recipes::CraftingRecipeKind};

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
    CalendarChanged(crate::state::calendar::CalendarTransport),
    BuildingsChanged(crate::state::buildings::BuildingTransport),
    FulfillmentsChanged(crate::state::recipes::FulfillmentTransport),
    ResourcesChanged(crate::state::resources::ResourceTransport),
    TimeChanged(crate::state::time::TimeTransport),
    VisibilityChanged(crate::state::ui::VisibilityTransport),
}
