use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use sorrow_core::communication::{EngineMessage, EngineUpdate};
use sorrow_engine::Endpoint;

use crate::store::{Global, GlobalStoreFields};

pub fn connect() -> (Endpoint, Store<Global>) {
    let store = Store::new(Global::default());
    let endpoint = Endpoint::new(move |message| update_store(store, message), "./engine.js");
    (endpoint, store)
}

pub fn provide_endpoint(endpoint: Endpoint) {
    provide_context(SendWrapper::new(Rc::new(endpoint)));
}

pub fn use_endpoint() -> SendWrapper<Rc<Endpoint>> {
    expect_context::<SendWrapper<Rc<Endpoint>>>()
}

fn update_store(store: Store<Global>, message: EngineMessage) {
    match message {
        EngineMessage::Loaded => tracing::info!("Loaded."),
        EngineMessage::Updated(updates) => {
            for update in updates {
                accept_update(store, update);
            }
            store.is_loaded().maybe_update(|v| {
                if *v {
                    false
                } else {
                    *v = true;
                    true
                }
            });
        }
    }
}

fn accept_update(store: Store<Global>, update: EngineUpdate) {
    match update {
        EngineUpdate::CalendarChanged(calendar) => {
            use crate::store::CalendarStoreFields;
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
        EngineUpdate::BuildingsChanged(state) => {
            use crate::store::BuildingStoreFields;
            for (kind, level) in state.levels.iter() {
                if let Some(level) = level {
                    store
                        .buildings()
                        .write_untracked()
                        .entry(*kind)
                        .and_modify(|e| e.level().set(*level));
                }
            }
        }
        EngineUpdate::FulfillmentsChanged(state) => {
            use crate::store::FulfillmentStoreFields;
            for (kind, fulfillment) in state.fulfillments.iter() {
                if let Some(fulfillment) = fulfillment {
                    store
                        .fulfillments()
                        .write_untracked()
                        .entry(*kind)
                        .and_modify(|e| e.fulfillment().set(*fulfillment));
                }
            }
        }
        EngineUpdate::ResourcesChanged(state) => {
            use crate::store::ResourceStoreFields;
            for (kind, amount) in state.amounts.iter() {
                if let Some(amount) = amount {
                    store
                        .resources()
                        .write_untracked()
                        .entry(*kind)
                        .and_modify(|e| e.amount().set(*amount));
                }
            }
            for (kind, delta) in state.deltas.iter() {
                if let Some(delta) = delta {
                    store
                        .resources()
                        .write_untracked()
                        .entry(*kind)
                        .and_modify(|e| e.delta().set(*delta));
                }
            }
        }
        EngineUpdate::TimeChanged(time) => {
            if let Some(running_state) = time.running_state {
                store.running_state().set(running_state);
            }
        }
        EngineUpdate::VisibilityChanged(state) => {
            use crate::store::UiStateStoreFields;
            for (id, visible) in state.nodes.iter() {
                if let Some(visible) = visible {
                    store
                        .ui()
                        .write_untracked()
                        .entry(*id)
                        .and_modify(|e| e.visible().set(*visible));
                }
            }
        }
    }
}
