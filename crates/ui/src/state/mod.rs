use std::collections::BTreeMap;

use leptos::prelude::*;
use reactive_stores::Store;

use sorrow_core::state as core_state;
use sorrow_core::state::{calendar::SeasonKind, precision::Precision, time::RunningState};

#[derive(Store)]
pub struct Building {
    pub kind: core_state::buildings::Kind,
    pub level: u32,
}

#[derive(Store)]
pub struct Calendar {
    pub day: i16,
    pub season: core_state::calendar::SeasonKind,
    pub year: usize,
}

#[derive(Store)]
pub struct Fulfillment {
    pub kind: core_state::recipes::Kind,
    pub fulfillment: core_state::recipes::Fulfillment,
}

#[derive(Store)]
pub struct Preferences {
    pub precision: Precision,
}

#[derive(Store)]
pub struct Resource {
    pub kind: core_state::resources::Kind,
    pub amount: f64,
    pub delta: f64,
}

#[derive(Store)]
pub struct UiState {
    pub id: core_state::ui::NodeId,
    pub visible: bool,
}

#[derive(Store)]
pub struct GlobalStore {
    pub buildings: BTreeMap<core_state::buildings::Kind, Store<Building>>,
    pub calendar: Calendar,
    pub fulfillments: BTreeMap<core_state::recipes::Kind, Store<Fulfillment>>,
    pub preferences: Preferences,
    pub resources: BTreeMap<core_state::resources::Kind, Store<Resource>>,
    pub running_state: RunningState,
    pub ui: BTreeMap<core_state::ui::NodeId, Store<UiState>>,
}

impl GlobalStore {
    fn new() -> Self {
        use core_state::buildings::Kind as BuildingKind;
        use core_state::recipes::Kind as RecipeKind;
        use core_state::resources::Kind as ResourceKind;

        fn buildings_map() -> BTreeMap<BuildingKind, Store<Building>> {
            <BuildingKind as core_state::KeyIter>::key_iter()
                .map(|kind| (kind, Store::new(Building { kind, level: 0 })))
                .collect()
        }
        fn resources_map() -> BTreeMap<ResourceKind, Store<Resource>> {
            <ResourceKind as core_state::KeyIter>::key_iter()
                .map(|kind| {
                    (
                        kind,
                        Store::new(Resource {
                            kind,
                            amount: 0.0,
                            delta: 0.0,
                        }),
                    )
                })
                .collect()
        }
        fn fulfillments_map() -> BTreeMap<RecipeKind, Store<Fulfillment>> {
            <RecipeKind as core_state::KeyIter>::key_iter()
                .map(|kind| {
                    (
                        kind,
                        Store::new(Fulfillment {
                            kind,
                            fulfillment: core_state::recipes::Fulfillment::Unfulfilled,
                        }),
                    )
                })
                .collect()
        }
        fn ui_map() -> BTreeMap<core_state::ui::NodeId, Store<UiState>> {
            <core_state::ui::NodeId as core_state::KeyIter>::key_iter()
                .map(|e| {
                    (
                        e,
                        Store::new(UiState {
                            id: e,
                            visible: false,
                        }),
                    )
                })
                .collect()
        }
        Self {
            buildings: buildings_map(),
            calendar: Calendar {
                day: 0,
                season: SeasonKind::Spring,
                year: 0,
            },
            fulfillments: fulfillments_map(),
            preferences: Preferences {
                precision: Precision::default(),
            },
            resources: resources_map(),
            running_state: RunningState::default(),
            ui: ui_map(),
        }
    }
}

pub fn provide_global_store() {
    provide_context(Store::new(GlobalStore::new()));
}

pub fn use_global_store() -> Store<GlobalStore> {
    expect_context::<Store<GlobalStore>>()
}
