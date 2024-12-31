use partially::Partial;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct Acceleration(f64);

impl Default for Acceleration {
    fn default() -> Self {
        Self(1f64)
    }
}

impl TryFrom<f64> for Acceleration {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let value = ((value * 10.0).round()) / 10.0;

        if value >= 0.1 {
            Ok(Self(value))
        } else {
            Err("Acceleration must be 0.1 or higher.")
        }
    }
}

impl From<Acceleration> for f64 {
    fn from(val: Acceleration) -> Self {
        val.0
    }
}

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
    pub acceleration: Acceleration,
}
