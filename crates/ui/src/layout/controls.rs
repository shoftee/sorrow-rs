use leptos::prelude::*;
use sorrow_core::communication::Intent;

use crate::{events::use_keyboard_events, state::send_command};

#[component]
pub fn ControlsContainer() -> impl IntoView {
    #[expect(unused_variables)]
    let keyboard_events = use_keyboard_events();

    let gather_catnip = move || send_command(Intent::GatherCatnip);
    let refine_catnip = move || send_command(Intent::RefineCatnip);

    view! {
        <section class="controls-area unscroll-y">
            <div class="grid grid-cols-2 gap-2">
                <div>
                    <Button command=gather_catnip>"Gather catnip"</Button>
                </div>
                <div>
                    <Button command=refine_catnip>"Refine catnip"</Button>
                </div>
                // <div>{move || keyboard_events.ctrl.get() }</div>
                // <div>{move || keyboard_events.shift.get() }</div>
                // <div>{move || keyboard_events.alt.get() }</div>
            </div>
        </section>
    }
}

#[component]
fn Button(command: fn(), children: ChildrenFn) -> impl IntoView {
    view! {
        <button class="btn btn-outline-secondary w-100" type="button" on:click=move |_| command()>
            {children()}
        </button>
    }
}
