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

    let state_signals = use_state_signals(cx);

    let catnip = Signal::derive(cx, move || state_signals.resource.catnip.get());

    view! { cx,
        <main class="unscrollable">
            <div class="nav-container">
                <div>"navigation goes here"</div>
                <div class="main-container unscrollable">
                    <div class="col unscrollable">
                        <div>"Resources"</div>
                        <div>"Catnip is: "<DecimalView value=catnip /></div>
                    </div>
                    <div class="col unscrollable"></div>
                    <div class="col env-container unscrollable">"Calendar and History"</div>
                </div>
            </div>
        </main>
    }
}
