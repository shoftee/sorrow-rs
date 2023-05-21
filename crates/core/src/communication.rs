use super::time::Acceleration;
use serde::{Deserialize, Serialize};
use sorrow_derive::Reactive;

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

#[derive(Debug, Default, Reactive)]
pub struct TimeState {
    pub paused: bool,
    pub acceleration: Acceleration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialTimeState {
    pub paused: Option<bool>,
    pub acceleration: Option<Acceleration>,
}

#[derive(Debug, Default, Reactive)]
pub struct ResourceState {
    pub catnip: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialResourceState {
    pub catnip: Option<f64>,
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
