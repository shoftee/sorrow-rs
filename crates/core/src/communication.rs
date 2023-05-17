use serde::{Deserialize, Serialize};

use super::time::Acceleration;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Command {
    Initialize,
    TimeControl(TimeControl),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum TimeControl {
    SetAcceleration(Acceleration),
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Notification {
    LogMessage(String),
    WarnMessage(String),
    Initialized,
    StateChanged {
        paused: bool,
        acceleration: Acceleration,
    },
}
