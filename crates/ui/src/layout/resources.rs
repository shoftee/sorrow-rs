use leptos::prelude::*;
use leptos_i18n::*;
use reactive_stores::Store;

use sorrow_core::state::{
    ui::{NodeId, ResourceNodeId},
    KeyIter,
};

use crate::{
    components::{conditional::*, numbers::DecimalView, strings::ResourceLabel},
    formatter::ShowSign,
    i18n::use_i18n,
    store::{
        use_global_store, GlobalStoreFields, Resource, ResourceStoreFields, UiStateStoreFields,
    },
};

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let i18n = use_i18n();

    let resources = resources();
    let has_resources = Memo::new(move |_| !resources.get().is_empty());

    let is_expanded = RwSignal::new(true);

    view! {
        <section class="resources-area unscroll-y">
            <div class="resources-container">
                <Conditional>
                    <Main slot condition=has_resources>
                        <div class="flex flex-col resource-list">
                            <button
                                on:click=move |_| is_expanded.update(|v| *v = !*v)
                                class="flex flex-row items-center resource-expander"
                            >
                                <div class="flex-1 text-start">{ t_string!(i18n, resources.section.label) }</div>
                                <div class="flex-none" prop:hidden=is_expanded><i class="bi bi-arrows-expand"></i></div>
                            </button>
                            <Conditional>
                                <Main slot condition=is_expanded>
                                    <For
                                        each={move || resources.get()}
                                        key=|item| item.resource().get()
                                        let:child
                                    >
                                        <ResourceItem item=child />
                                    </For>
                                </Main>
                            </Conditional>
                        </div>
                    </Main>
                    <Fallback slot>
                        <div class="rounded p-1 border border-solid border-neutral-400">
                            { t_string!(i18n, resources.section.empty) }
                        </div>
                    </Fallback>
                </Conditional>
            </div>
        </section>
    }
}

#[component]
fn ResourceItem(#[prop(into)] item: Store<crate::store::Resource>) -> impl IntoView {
    let amount = Memo::new(move |_| item.amount().get());
    let delta = Memo::new(move |_| item.delta().get());

    view! {
        <li class="text-xs">
            <ResourceLabel resource=item.resource().get() />
            " "
            <DecimalView value=amount />
            " "
            <DecimalView value=delta show_sign=ShowSign::Always />
        </li>
    }
}

fn resources() -> Signal<Vec<Store<Resource>>> {
    let ui_store = use_global_store().ui();
    let resources_store = use_global_store().resources();

    Signal::derive(move || {
        ResourceNodeId::key_iter()
            .filter_map(|id| {
                if ui_store
                    .read_untracked()
                    .get(&NodeId::Resources(id))
                    .unwrap()
                    .visible()
                    .get()
                {
                    Some(
                        *resources_store
                            .read_untracked()
                            .get(&id.into())
                            .expect("Could not find resource for node"),
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    })
}
