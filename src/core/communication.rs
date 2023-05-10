use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Command {
    Initialize,
    Start,
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Notification {
    LogMessage(String),
    WarnMessage(String),
    Initialized,
    Started,
    Paused,
    StateChanged { id: u64 },
}
