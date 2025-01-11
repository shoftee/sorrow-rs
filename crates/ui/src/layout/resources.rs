use leptos::prelude::*;
use reactive_stores::Store;
use sorrow_core::state::resources::Kind;

use crate::components::{conditional::*, numbers::DecimalView};
use crate::formatter::ShowSign;
use crate::state::{use_global_store, GlobalStoreStoreFields, ResourceStoreFields};

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let resources = Signal::derive(|| {
        use_global_store()
            .resources()
            .read()
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<_>>()
    });
    let has_resources = Memo::new(move |_| {
        resources
            .get()
            .iter()
            .any(|resource| resource.amount().get() > 0.0)
    });

    let expanded_rw = RwSignal::new(true);

    view! {
        <section class="resources-area unscroll-y">
            <div class="resources-container">
                <Conditional>
                    <Main slot condition=has_resources>
                        <ul class="list-group resources-list">
                            <ResourceExpander expanded=expanded_rw />
                            <Conditional>
                                <Main slot condition=expanded_rw>
                                    <For each={move || resources.get()} key=|item| item.kind().get() let:child>
                                        <ResourceItem item=child />
                                    </For>
                                </Main>
                            </Conditional>
                        </ul>
                    </Main>
                    <Fallback slot>
                        <NoResources />
                    </Fallback>
                </Conditional>
            </div>
        </section>
    }
}

#[component]
fn ResourceItem(#[prop(into)] item: Store<crate::state::Resource>) -> impl IntoView {
    let label = match item.kind().get() {
        Kind::Catnip => "catnip",
    };

    let amount = Memo::new(move |_| item.amount().get());
    let delta = Memo::new(move |_| item.delta().get());

    view! {
        <li class="list-group-item small">
            {label}
            " "
            <DecimalView value=amount />
            " "
            <DecimalView value=delta show_sign=ShowSign::Always />
        </li>
    }
}

#[component]
fn ResourceExpander(expanded: RwSignal<bool>) -> impl IntoView {
    let collapsed = Memo::new(move |_| !expanded.get());

    view! {
        <button
            on:click=move |_| expanded.update(|v| *v = !*v)
            class="list-group-item list-group-item-action expander"
        >
            <div>"Resources"</div>
            <Conditional>
                <Main slot condition=collapsed>
                    <div><i class="bi bi-arrows-expand"></i></div>
                </Main>
            </Conditional>
        </button>
    }
}

#[component]
fn NoResources() -> impl IntoView {
    view! { <li class="list-group-item">"Your paws are empty."</li> }
}
