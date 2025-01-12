use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    GatherCatnip,
    RefineCatnip,
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
    ResourcesChanged(crate::state::resources::ResourceState),
    TimeChanged(crate::state::time::PartialTimeState),
}
