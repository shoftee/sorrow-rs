#![deny(
    clippy::all,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod actor;
mod codec;
mod traits;

pub use actor::*;
pub use codec::{Bincode, Codec};
pub use traits::*;
