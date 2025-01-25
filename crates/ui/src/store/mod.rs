use std::collections::BTreeMap;

use leptos::prelude::*;
use reactive_stores::Store;

use sorrow_core::state::{
    buildings::BuildingKind,
    calendar::SeasonKind,
    precision::Precision,
    recipes::{FulfillmentState, RecipeKind},
    resources::ResourceKind,
    time::RunningState,
    ui::NodeId,
    KeyIter,
};

#[derive(Store)]
pub struct Building {
    pub kind: BuildingKind,
    pub level: u32,
}

#[derive(Store)]
pub struct Calendar {
    pub day: i16,
    pub season: SeasonKind,
    pub year: usize,
}

#[derive(Store)]
pub struct Fulfillment {
    pub kind: RecipeKind,
    pub fulfillment: FulfillmentState,
}

#[derive(Store)]
pub struct Preferences {
    pub precision: Precision,
}

#[derive(Store)]
pub struct Resource {
    pub kind: ResourceKind,
    pub amount: f64,
    pub delta: f64,
}

#[derive(Store)]
pub struct UiState {
    pub id: NodeId,
    pub visible: bool,
}

#[derive(Store)]
pub struct Global {
    pub is_loaded: bool,

    pub buildings: BTreeMap<BuildingKind, Store<Building>>,
    pub calendar: Calendar,
    pub fulfillments: BTreeMap<RecipeKind, Store<Fulfillment>>,
    pub preferences: Preferences,
    pub resources: BTreeMap<ResourceKind, Store<Resource>>,
    pub running_state: RunningState,
    pub ui: BTreeMap<NodeId, Store<UiState>>,
}

impl Default for Global {
    fn default() -> Self {
        fn buildings_map() -> BTreeMap<BuildingKind, Store<Building>> {
            <BuildingKind as KeyIter>::key_iter()
                .map(|kind| (kind, Store::new(Building { kind, level: 0 })))
                .collect()
        }
        fn resources_map() -> BTreeMap<ResourceKind, Store<Resource>> {
            <ResourceKind as KeyIter>::key_iter()
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
            <RecipeKind as KeyIter>::key_iter()
                .map(|kind| {
                    (
                        kind,
                        Store::new(Fulfillment {
                            kind,
                            fulfillment: FulfillmentState::Unfulfilled,
                        }),
                    )
                })
                .collect()
        }
        fn ui_map() -> BTreeMap<NodeId, Store<UiState>> {
            <NodeId as KeyIter>::key_iter()
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
            is_loaded: false,

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

pub fn provide_store(store: Store<Global>) {
    provide_context(store);
}

pub fn use_global_store() -> Store<Global> {
    expect_context::<Store<Global>>()
}
