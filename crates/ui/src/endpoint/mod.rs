use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use sorrow_core::communication::{EngineMessage, EngineUpdate};
use sorrow_engine::Endpoint;

use crate::store::{Global, GlobalStoreFields, IngredientFulfillmentStoreFields};

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
            for (building, level) in state.levels.iter() {
                if let Some(level) = level {
                    store
                        .buildings()
                        .write_untracked()
                        .entry(*building)
                        .and_modify(|e| e.level().set(*level));
                }
            }
        }
        EngineUpdate::FulfillmentsChanged(state) => {
            use crate::store::FulfillmentStoreFields;
            for (recipe, fulfillment) in state.fulfillments.iter() {
                if let Some(fulfillment) = fulfillment {
                    store
                        .fulfillments()
                        .write_untracked()
                        .entry(*recipe)
                        .and_modify(|e| e.fulfillment().set(*fulfillment));
                }
            }

            for ((recipe, resource), required_amount) in state.required_amounts.iter() {
                if let Some(required_amount) = required_amount {
                    store
                        .fulfillments()
                        .write_untracked()
                        .entry(*recipe)
                        .and_modify(|e| {
                            e.ingredients()
                                .write_untracked()
                                .entry(*resource)
                                .and_modify(|e| e.required_amount().set(*required_amount));
                        });
                }
            }
        }
        EngineUpdate::ResourcesChanged(state) => {
            use crate::store::ResourceStoreFields;
            for (resource, amount) in state.amounts.iter() {
                if let Some(amount) = amount {
                    store
                        .resources()
                        .write_untracked()
                        .entry(*resource)
                        .and_modify(|e| e.amount().set(*amount));
                }
            }
            for (resource, delta) in state.deltas.iter() {
                if let Some(delta) = delta {
                    store
                        .resources()
                        .write_untracked()
                        .entry(*resource)
                        .and_modify(|e| e.delta().set(*delta));
                }
            }
            for (resource, capacity) in state.capacities.iter() {
                if let Some(capacity) = capacity {
                    store
                        .resources()
                        .write_untracked()
                        .entry(*resource)
                        .and_modify(|e| e.capacity().set(*capacity));
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
            for (node, visible) in state.nodes.iter() {
                if let Some(visible) = visible {
                    store
                        .ui()
                        .write_untracked()
                        .entry(*node)
                        .and_modify(|e| e.visible().set(*visible));
                }
            }
        }
    }
}
