use serde::{Deserialize, Serialize};

use crate::state::{Acceleration, PartialState};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    LogMessage(String),
    WarnMessage(String),
    Initialized,
    StateChanged(PartialState),
}
