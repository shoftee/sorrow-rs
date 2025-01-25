use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeasonKind {
    Spring,
    Summer,
    Autumn,
    Winter,
}
