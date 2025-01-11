use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;
use sorrow_core::communication::{Intent, Notification};
use sorrow_engine::Endpoint;

use crate::state::{
    use_global_store, BuildingsStoreFields, CalendarStoreFields, GlobalStore,
    GlobalStoreStoreFields, ResourceStoreFields,
};

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
                    .get_state(&sorrow_core::state::buildings::Kind::CatnipField)
                {
                    store.buildings().catnip_fields().set(*catnip_fields);
                }
            }
            if let Some(resources) = state.resources {
                for (kind, amount) in resources.amounts.iter() {
                    if let Some(amount) = amount {
                        store
                            .resources()
                            .write_untracked()
                            .entry(*kind)
                            .and_modify(|e| e.amount().set(*amount));
                    }
                }
                for (kind, delta) in resources.deltas.iter() {
                    if let Some(delta) = delta {
                        store
                            .resources()
                            .write_untracked()
                            .entry(*kind)
                            .and_modify(|e| e.delta().set(*delta));
                    }
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
