use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeasonKind {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde_with::skip_serializing_none]
pub struct PartialCalendarState {
    pub day: Option<i16>,
    pub season: Option<SeasonKind>,
    pub year: Option<usize>,
}
