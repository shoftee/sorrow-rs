use std::rc::Rc;

use leptos::prelude::*;
use reactive_stores::Store;
use send_wrapper::SendWrapper;

use sorrow_core::communication::{Intent, Notification};
use sorrow_engine::Endpoint;

use crate::state::{use_global_store, GlobalStore, GlobalStoreStoreFields};

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

fn accept(store: Store<GlobalStore>, notifications: Vec<Notification>) {
    use Notification::*;

    for notification in notifications {
        match notification {
            Initialized => tracing::debug!("World initialized."),
            CalendarChanged(calendar) => {
                use crate::state::CalendarStoreFields;
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
            BuildingsChanged(state) => {
                use crate::state::BuildingStoreFields;
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
            FulfillmentsChanged(state) => {
                use crate::state::FulfillmentStoreFields;
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
            ResourcesChanged(state) => {
                use crate::state::ResourceStoreFields;
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
            TimeChanged(time) => {
                if let Some(running_state) = time.running_state {
                    store.running_state().set(running_state);
                }
            }
            VisibilityChanged(state) => {
                use crate::state::UiStateStoreFields;
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
}
