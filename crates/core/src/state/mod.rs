pub mod calendar;
pub mod options;
pub mod precision;
pub mod resources;
pub mod time;

use std::{collections::HashMap, hash::Hash};

use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PartialState {
    pub time: Option<time::PartialTimeState>,
    pub resources: Option<resources::ResourceState>,
    pub calendar: Option<calendar::PartialCalendarState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateTable<K, V>(HashMap<K, Option<V>, ahash::RandomState>)
where
    K: Eq + Hash + IntoEnumIterator;

impl<K, V> StateTable<K, V>
where
    K: Eq + Hash + IntoEnumIterator,
{
    pub fn new() -> Self {
        Self(<K as IntoEnumIterator>::iter().map(|k| (k, None)).collect())
    }

    pub fn get_state(&self, key: &K) -> &Option<V> {
        self.0.get(key).unwrap()
    }

    pub fn get_state_mut(&mut self, key: &K) -> &mut Option<V> {
        self.0.get_mut(key).unwrap()
    }
}

impl<K, V> Default for StateTable<K, V>
where
    K: Eq + Hash + IntoEnumIterator,
{
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! state_key {
    { $vis:vis enum $ident:ident $tt:tt } => {
        #[derive(
            ::std::fmt::Debug,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::std::cmp::PartialEq,
            ::std::cmp::Eq,
            ::std::hash::Hash,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::strum::EnumIter,
        )]
        $vis enum $ident $tt
    };
}
