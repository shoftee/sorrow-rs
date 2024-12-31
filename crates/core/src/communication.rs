use serde::{Deserialize, Serialize};

use crate::state::{Acceleration, PartialResourceState, RunningState};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    GatherCatnip,
    RefineCatnip,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeControl {
    SetAcceleration(Acceleration),
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PartialState {
    pub acceleration: Option<Acceleration>,
    pub running_state: Option<RunningState>,
    pub resource: Option<PartialResourceState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    LogMessage(String),
    WarnMessage(String),
    Initialized,
    StateChanged(PartialState),
}
