use serde::{Deserialize, Serialize};

use crate::state::{Acceleration, PartialResourceState, RunningState};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Initialize,
    TimeControl(TimeControl),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeControl {
    SetAcceleration(Acceleration),
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
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
