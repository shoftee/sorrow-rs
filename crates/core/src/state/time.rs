use partially::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum RunningState {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Default, Partial)]
#[partially(attribute(derive(Default, Debug, Serialize, Deserialize)))]
pub struct TimeState {
    pub running_state: RunningState,
}
