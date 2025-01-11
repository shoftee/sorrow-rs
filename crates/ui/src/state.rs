use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;
use sorrow_core::state as core_state;
use sorrow_core::{
    communication::{Intent, Notification},
    state::{calendar::SeasonKind, precision::Precision, time::RunningState},
};
use sorrow_engine::Endpoint;

#[derive(Store)]
pub struct Preferences {
    pub precision: Precision,
}

#[derive(Store)]
pub struct Resources {
    pub catnip: f64,
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
    pub resources: Resources,
}

impl GlobalStore {
    fn new() -> Self {
        Self {
            preferences: Preferences {
                precision: Precision::default(),
            },
            running_state: RunningState::default(),
            buildings: Buildings { catnip_fields: 0 },
            calendar: Calendar {
                day: 0,
                season: SeasonKind::Spring,
                year: 0,
            },
            resources: Resources { catnip: 0.0 },
        }
    }
}

fn accept(store: Store<GlobalStore>, notification: Notification) {
    use Notification::*;

    match notification {
        Initialized => tracing::debug!("World initialized."),
        StateChanged(state) => {
            if let Some(time) = state.time {
                if let Some(running_state) = time.running_state {
                    store.running_state().set(running_state);
                }
            }
            if let Some(buildings) = state.buildings {
                if let Some(catnip_fields) = buildings
                    .levels
                    .get_state(&core_state::buildings::Kind::CatnipField)
                {
                    store.buildings().catnip_fields().set(*catnip_fields);
                }
            }
            if let Some(resources) = state.resources {
                if let Some(catnip) = resources
                    .amounts
                    .get_state(&core_state::resources::Kind::Catnip)
                {
                    store.resources().catnip().set(*catnip);
                }
            }
            if let Some(calendar) = state.calendar {
                if let Some(day) = calendar.day {
                    store.calendar().day().set(day);
                }
                if let Some(season) = calendar.season {
                    store.calendar().season().set(season);
                }
                if let Some(year) = calendar.year {
                    store.calendar().year().set(year);
                }
            }
        }
    }
}

pub fn provide_global_store() {
    provide_context(Store::new(GlobalStore::new()));
}

pub fn provide_endpoint_context() {
    let global_store = use_global_store();
    let endpoint = Rc::new(Endpoint::new(
        move |notification| accept(global_store, notification),
        "./engine.js",
    ));
    let endpoint_wrapped = SendWrapper::new(endpoint.clone());
    provide_context(endpoint_wrapped);

    endpoint.send(Intent::Load);
}

pub fn use_endpoint() -> SendWrapper<Rc<Endpoint>> {
    expect_context::<SendWrapper<Rc<Endpoint>>>()
}

pub fn use_global_store() -> Store<GlobalStore> {
    expect_context::<Store<GlobalStore>>()
}
