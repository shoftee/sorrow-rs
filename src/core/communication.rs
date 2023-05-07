use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    Initialize,
    Increment,
}

#[derive(Serialize, Deserialize)]
pub enum Notification {
    LogMessage(String),
    Delta { id: u64 },
}
