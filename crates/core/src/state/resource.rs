use partially::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Partial)]
#[partially(attribute(derive(Default, Debug, Serialize, Deserialize)))]
pub struct ResourceState {
    pub catnip: f64,
}
