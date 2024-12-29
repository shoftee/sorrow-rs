use leptos::prelude::*;

use crate::events::use_keyboard_events;

#[component]
pub fn ControlsContainer() -> impl IntoView {
    let keyboard_events = use_keyboard_events();
    view! {
        <div class="container controls-container">
            <div class="row">
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button">"Gather catnip"</button>
                </div>
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button">"Refine catnip"</button>
                </div>
            </div>
            <div class="row">
            <div class="col">{move || keyboard_events.ctrl.get() }</div>
            <div class="col">{move || keyboard_events.shift.get() }</div>
            <div class="col">{move || keyboard_events.alt.get() }</div>
            </div>
        </div>
    }
}
