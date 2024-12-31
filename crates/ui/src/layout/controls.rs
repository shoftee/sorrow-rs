use leptos::prelude::*;
use sorrow_core::communication::Command;

use crate::{events::use_keyboard_events, state::send_command};

#[component]
pub fn ControlsContainer() -> impl IntoView {
    let keyboard_events = use_keyboard_events();

    let gather_catnip = move |_| send_command(Command::GatherCatnip);
    let refine_catnip = move |_| send_command(Command::RefineCatnip);

    view! {
        <div class="container controls-container">
            <div class="row">
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button" on:click=gather_catnip>"Gather catnip"</button>
                </div>
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button" on:click=refine_catnip>"Refine catnip"</button>
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
