use serde::{Deserialize, Serialize};

use crate::state::{Acceleration, PartialResourceState, PartialTimeState};

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
pub enum Notification {
    LogMessage(String),
    WarnMessage(String),
    Initialized,
    StateChanged {
        time: Option<PartialTimeState>,
        resource: Option<PartialResourceState>,
    },
}
