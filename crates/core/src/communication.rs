use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    GatherCatnip,
    RefineCatnip,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeControl {
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    Initialized,
    StateChanged(crate::state::PartialState),
}
