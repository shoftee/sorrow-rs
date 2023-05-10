use leptos::*;
use leptos_reactive::Scope;

use crate::ui::{events, state};

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
    let keyboard_events = events::use_keyboard_events(cx);
    create_effect(cx, move |_| log!("Ctrl: {}", keyboard_events.ctrl.get()));
    create_effect(cx, move |_| log!("Shift: {}", keyboard_events.shift.get()));
    create_effect(cx, move |_| log!("Alt: {}", keyboard_events.alt.get()));

    let id = state::use_state_signals(cx).id();
    view! { cx,
        <main class="unscrollable">
            <div class="nav-container">
                <div>"navigation goes here"</div>
                <div class="main-container unscrollable">
                    <div class="col unscrollable">"Resources"</div>
                    <div class="col unscrollable">"ID is: "{id}</div>
                    <div class="col env-container unscrollable">"Calendar and History"</div>
                </div>
            </div>
        </main>
    }
}
