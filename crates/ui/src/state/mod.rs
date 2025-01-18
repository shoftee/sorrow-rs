use std::collections::BTreeMap;

use leptos::prelude::*;
use reactive_stores::Store;
use sorrow_core::state as core_state;
use sorrow_core::state::{calendar::SeasonKind, precision::Precision, time::RunningState};

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
pub struct Fulfillment {
    pub kind: core_state::recipes::Kind,
    pub fulfillment: core_state::recipes::Fulfillment,
}

#[derive(Store)]
pub struct Buildings {
    pub catnip_fields: u32,
}

#[derive(Store)]
pub struct Calendar {
    pub day: i16,
    pub season: SeasonKind,
    pub year: usize,
}

#[derive(Store)]
pub struct GlobalStore {
    pub preferences: Preferences,
    pub running_state: RunningState,
    pub calendar: Calendar,
    pub buildings: Buildings,
    pub fulfillments: BTreeMap<core_state::recipes::Kind, Store<Fulfillment>>,
    pub resources: BTreeMap<core_state::resources::Kind, Store<Resource>>,
}

impl GlobalStore {
    fn new() -> Self {
        fn resources_map() -> BTreeMap<core_state::resources::Kind, Store<crate::state::Resource>> {
            core_state::resources::Kind::iter()
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
        fn fulfillments_map(
        ) -> BTreeMap<core_state::recipes::Kind, Store<crate::state::Fulfillment>> {
            core_state::recipes::Kind::iter()
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
        Self {
            preferences: Preferences {
                precision: Precision::default(),
            },
            running_state: RunningState::default(),
            buildings: Buildings { catnip_fields: 0 },
            fulfillments: fulfillments_map(),
            calendar: Calendar {
                day: 0,
                season: SeasonKind::Spring,
                year: 0,
            },
            resources: resources_map(),
        }
    }
}

pub fn provide_global_store() {
    provide_context(Store::new(GlobalStore::new()));
}

pub fn use_global_store() -> Store<GlobalStore> {
    expect_context::<Store<GlobalStore>>()
}
