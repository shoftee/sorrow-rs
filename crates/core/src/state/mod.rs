pub mod buildings;
pub mod calendar;
pub mod options;
pub mod precision;
pub mod recipes;
pub mod resources;
pub mod time;

use std::{
    collections::{hash_map::Iter, HashMap},
    hash::Hash,
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

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

    pub fn iter(&self) -> StateTableIter<K, V> {
        StateTableIter {
            inner: self.0.iter(),
        }
    }
}

pub struct StateTableIter<'a, K, V>
where
    K: Eq + Hash + IntoEnumIterator,
{
    inner: Iter<'a, K, Option<V>>,
}

impl<'a, K, V> Iterator for StateTableIter<'a, K, V>
where
    K: Eq + Hash + IntoEnumIterator,
{
    type Item = (&'a K, &'a Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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
            ::std::cmp::PartialOrd,
            ::std::cmp::Ord,
            ::std::hash::Hash,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::strum::EnumIter,
        )]
        $vis enum $ident $tt

        impl $ident {
            $vis fn iter() -> <$ident as ::strum::IntoEnumIterator>::Iterator {
                <$ident as ::strum::IntoEnumIterator>::iter()
            }
        }
    };
}
