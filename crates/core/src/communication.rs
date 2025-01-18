use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    QueueWorkOrder(WorkOrderKind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkOrderKind {
    Craft(crate::state::recipes::Crafting),
    Construct(crate::state::buildings::Kind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeControl {
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    Initialized,
    CalendarChanged(crate::state::calendar::PartialCalendarState),
    BuildingsChanged(crate::state::buildings::BuildingState),
    FulfillmentsChanged(crate::state::recipes::FulfillmentState),
    ResourcesChanged(crate::state::resources::ResourceState),
    TimeChanged(crate::state::time::PartialTimeState),
}
