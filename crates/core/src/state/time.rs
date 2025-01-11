use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum RunningState {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialTimeState {
    pub running_state: Option<RunningState>,
}
