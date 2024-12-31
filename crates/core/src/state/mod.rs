mod options;
mod precision;
mod resource;
mod time;

use serde::Deserialize;
use serde::Serialize;

pub use self::options::*;
pub use self::precision::*;
pub use self::resource::*;
pub use self::time::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PartialState {
    pub time: time::PartialTimeState,
    pub resource: resource::PartialResourceState,
}
