use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct ResourceState {
    pub catnip: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialResourceState {
    pub catnip: Option<f64>,
}
