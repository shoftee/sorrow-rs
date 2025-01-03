use std::{default::Default, hash::Hash, marker::PhantomData};

use bevy::{
    app::{App, Plugin},
    ecs::{
        query::{QueryData, QueryEntityError, WorldQuery},
        system::SystemParam,
    },
    prelude::{Component, Entity, OnAdd, OnRemove, Query, Res, ResMut, Resource, Trigger, With},
    utils::HashMap,
};

pub struct LookupIndexPlugin<K>
where
    K: Component + Eq + Hash + Clone,
{
    _phantom: PhantomData<K>,
}

impl<K> LookupIndexPlugin<K>
where
    K: Component + Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::<K>,
        }
    }
}

#[derive(Resource, Default)]
pub struct LookupIndex<K>
where
    K: Component + Eq + Hash + Clone,
{
    inner: HashMap<K, Entity>,
}

impl<K> LookupIndex<K>
where
    K: Component + Eq + Hash + Clone,
{
    pub fn get(&self, key: &K) -> Option<&Entity> {
        self.inner.get(key)
    }
}

impl<K> Plugin for LookupIndexPlugin<K>
where
    K: Component + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(LookupIndex {
            inner: HashMap::<K, Entity>::default(),
        })
        .add_observer(
            |trigger: Trigger<OnAdd, K>, data: Query<&K>, mut index: ResMut<LookupIndex<K>>| {
                let entity = trigger.entity();
                if let Ok(key) = data.get(entity) {
                    index.inner.insert(key.clone(), entity);
                } else {
                    bevy::log::warn!("Could not find entity to add to index.")
                }
            },
        )
        .add_observer(
            |trigger: Trigger<OnRemove, K>, data: Query<&K>, mut index: ResMut<LookupIndex<K>>| {
                let entity = trigger.entity();
                if let Ok(key) = data.get(entity) {
                    index.inner.remove(key);
                } else {
                    bevy::log::warn!("Could not find entity to remove from index.")
                }
            },
        );
    }
}

// #[derive(SystemParam)]
// pub struct IndexedQuery<'w, 's, K, D>
// where
//     K: Component + Eq + Hash + Clone,
//     D: 'static + QueryData,
// {
//     lookup: Res<'w, LookupIndex<K>>,
//     query: Query<'w, 's, D, With<K>>,
// }

// impl<K, D> IndexedQuery<'_, '_, K, D>
// where
//     K: Component + Eq + Hash + Clone,
//     D: 'static + QueryData,
// {
//     pub fn get(
//         &self,
//         key: K,
//     ) -> Result<<<D as QueryData>::ReadOnly as WorldQuery>::Item<'_>, IndexedLookupError> {
//         if let Some(entity) = self.lookup.inner.get(&key) {
//             match self.query.get(*entity) {
//                 Ok(value) => Ok(value),
//                 Err(QueryEntityError::NoSuchEntity(_)) => Err(IndexedLookupError::ValueNotFound),
//                 Err(err) => panic!("Unexpected query error: {}", err),
//             }
//         } else {
//             Err(IndexedLookupError::KeyNotFound)
//         }
//     }

//     pub fn get_all_indexed(
//         &self,
//     ) -> impl Iterator<Item = <<D as QueryData>::ReadOnly as WorldQuery>::Item<'_>> {
//         self.lookup
//             .inner
//             .values()
//             .filter_map(|v| match self.query.get(*v) {
//                 Ok(value) => Some(value),
//                 _ => None,
//             })
//     }
// }

#[derive(SystemParam)]
pub struct IndexedQueryMut<'w, 's, K, D>
where
    K: Component + Eq + Hash + Clone,
    D: 'static + QueryData,
{
    lookup: Res<'w, LookupIndex<K>>,
    query: Query<'w, 's, D, With<K>>,
}

impl<K, D> IndexedQueryMut<'_, '_, K, D>
where
    K: Component + Eq + Hash + Clone,
    D: 'static + QueryData,
{
    pub fn get_mut(&mut self, key: K) -> Result<<D as WorldQuery>::Item<'_>, IndexedLookupError> {
        if let Some(entity) = self.lookup.get(&key) {
            match self.query.get_mut(*entity) {
                Ok(value) => Ok(value),
                Err(QueryEntityError::NoSuchEntity(_)) => Err(IndexedLookupError::ValueNotFound),
                Err(err) => panic!("Unexpected query error: {}", err),
            }
        } else {
            Err(IndexedLookupError::KeyNotFound)
        }
    }
}

#[derive(Debug)]
pub enum IndexedLookupError {
    KeyNotFound,
    ValueNotFound,
}
