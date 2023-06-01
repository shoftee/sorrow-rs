use serde::{Deserialize, Serialize};
use sorrow_derive::Reactive;

#[derive(Debug, Default, Reactive)]
pub struct ResourceState {
    pub catnip: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartialResourceState {
    pub catnip: Option<f64>,
}
