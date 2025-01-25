use std::collections::{hash_map::Iter, HashMap};
use std::hash::Hash;

use ahash::RandomState;
use serde::{Deserialize, Serialize};

use crate::state::KeyIter;

#[derive(Debug, Serialize, Deserialize)]
pub struct StateTable<K, V>(HashMap<K, Option<V>, RandomState>)
where
    K: Eq + Hash + KeyIter<Item = K>;

impl<K, V> StateTable<K, V>
where
    K: Eq + Hash + KeyIter<Item = K>,
{
    pub fn new() -> Self {
        Self(<K as KeyIter>::key_iter().map(|k| (k, None)).collect())
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
    K: Eq + Hash,
{
    inner: Iter<'a, K, Option<V>>,
}

impl<'a, K, V> Iterator for StateTableIter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<K, V> Default for StateTable<K, V>
where
    K: Eq + Hash + KeyIter<Item = K>,
{
    fn default() -> Self {
        Self::new()
    }
}
