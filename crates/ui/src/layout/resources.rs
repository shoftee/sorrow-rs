use leptos::prelude::*;

use crate::conditional::*;
use crate::number_view::*;

use crate::state::use_state_signals;

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let state_signals = use_state_signals();

    let catnip = Signal::derive(move || state_signals.resource.catnip.get());

    let expanded_rw = RwSignal::new(true);

    view! {
        <div class="resources-container">
            <ul class="list-group resources-list">
                <ResourceExpander expanded=expanded_rw />
                <Conditional>
                    <Main slot condition=expanded_rw>
                        <li class="list-group-item small">"catnip " <DecimalView value=catnip /></li>
                    </Main>
                </Conditional>
            </ul>
        </div>
    }
}

#[component]
fn ResourceExpander(expanded: RwSignal<bool>) -> impl IntoView {
    let collapsed = Signal::derive(move || !expanded.get());

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
