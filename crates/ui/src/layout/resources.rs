use leptos::prelude::*;

use crate::conditional::*;
use crate::number_view::*;
use crate::state;

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let catnip_amount = state::with_state_signal(|s| s.resources.catnip).read_only();
    let has_resources = Signal::derive(move || catnip_amount.get() > 0.0);

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
                                    <li class="list-group-item small">"catnip " <DecimalView value=catnip_amount /></li>
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
