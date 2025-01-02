use std::{default::Default, hash::Hash, marker::PhantomData};

use bevy::{
    ecs::{
        query::{QueryData, QueryEntityError, WorldQuery},
        system::SystemParam,
    },
    prelude::{Component, Entity, OnAdd, OnRemove, Query, Res, ResMut, Resource, Trigger, With},
};

pub struct LookupIndexPlugin<K: Component + Eq + Hash> {
    _phantom: PhantomData<K>,
}

impl<K: Component + Eq + Hash> LookupIndexPlugin<K> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::<K>,
        }
    }
}

#[derive(Resource, Default)]
pub struct LookupIndex<K: Component + Eq + Hash + Clone> {
    inner: bevy::utils::HashMap<K, Entity>,
}

impl<K: Component + Eq + Hash + Clone> LookupIndex<K> {
    pub fn get(&self, key: &K) -> Option<&Entity> {
        self.inner.get(key)
    }
}

impl<K: Component + Eq + Hash + Clone> bevy::app::Plugin for LookupIndexPlugin<K> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(LookupIndex {
            inner: bevy::utils::HashMap::<K, Entity>::default(),
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
// pub struct IndexedQuery<'w, 's, K: Component + Eq + Hash + Clone, D: 'static + QueryData> {
//     lookup: Res<'w, LookupIndex<K>>,
//     query: Query<'w, 's, D, With<K>>,
// }

// impl<K: Component + Eq + Hash + Clone, D: 'static + QueryData> IndexedQuery<'_, '_, K, D> {
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
pub struct IndexedQueryMut<'w, 's, K: Component + Eq + Hash + Clone, D: 'static + QueryData> {
    lookup: Res<'w, LookupIndex<K>>,
    query: Query<'w, 's, D, With<K>>,
}

impl<K: Component + Eq + Hash + Clone, D: 'static + QueryData> IndexedQueryMut<'_, '_, K, D> {
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
