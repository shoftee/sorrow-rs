use leptos::*;

use crate::{events::use_keyboard_events, state::use_state_signals};

use crate::number_view::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="row">
            <div class="col">
                <div class="header-start">
                    <div>"Observable Sorrow"</div>
                    <div class="badge bg-success">
                        <i class="bi bi-droplet"></i>
                        " β "
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer>
            <div>
                "Observable Sorrow is a clone of "
                <a href="https://kittensgame.com/web/">"Kittens Game"</a> "."
            </div>
        </footer>
    }
}

#[component]
pub fn Center(cx: Scope) -> impl IntoView {
    let _keyboard_events = use_keyboard_events(cx);

    view! { cx,
        <main class="unscrollable">
            <div class="nav-container">
                <div>"navigation goes here"</div>
                <div class="main-container unscrollable">
                    <div class="col resources-col unscrollable">
                        <ResourcesContainer />
                    </div>
                    <div class="col controls-col unscrollable"></div>
                    <div class="col environment-col unscrollable">
                        <div class="env-container">"Calendar and History"</div>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn ResourcesContainer(cx: Scope) -> impl IntoView {
    let state_signals = use_state_signals(cx);

    let catnip = Signal::derive(cx, move || state_signals.resource.catnip.get());

    view! { cx,
        <div class="resources-container">
        <ul class="list-group resources-list">
            <button class="list-group-item list-group-item-action expander">
                <div>"Resources"</div>
                <div><i class="bi bi-arrows-expand"></i></div>
            </button>
            <li class="list-group-item small">"catnip " <DecimalView value=catnip /></li>
        </ul>
        </div>
    }
}

#[component]
fn NoResources(cx: Scope) -> impl IntoView {
    view! { cx, <li class="list-group-item">"Your paws are empty."</li> }
}
