use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Intent {
    /// Stub for initializing game session.
    Load,
    TimeControl(TimeControl),
    GatherCatnip,
    RefineCatnip,
    Build(crate::state::buildings::Kind),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TimeControl {
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    Initialized,
    StateChanged(Box<crate::state::PartialState>),
}
