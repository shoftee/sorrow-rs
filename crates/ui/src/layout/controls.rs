use leptos::prelude::*;
use sorrow_core::communication::Intent;
use sorrow_core::state::buildings;

use crate::components::Button;
use crate::events::use_keyboard_events;

#[component]
pub fn ControlsContainer() -> impl IntoView {
    view! {
        <section class="controls-area unscroll-y">
            <BonfireControls />
        </section>
    }
}

#[component]
fn BonfireControls() -> impl IntoView {
    #[expect(unused_variables)]
    let keyboard_events = use_keyboard_events();

    view! {
        <div class="controls grid grid-cols-2 gap-2">
            <Button intent=Intent::GatherCatnip>"Gather catnip"</Button>
            <Button intent=Intent::RefineCatnip>"Refine catnip"</Button>
            <Button intent=Intent::Build(buildings::Kind::CatnipField)>"Catnip field"</Button>
            // <div>{move || keyboard_events.ctrl.get() }</div>
            // <div>{move || keyboard_events.shift.get() }</div>
            // <div>{move || keyboard_events.alt.get() }</div>
        </div>
    }
}
