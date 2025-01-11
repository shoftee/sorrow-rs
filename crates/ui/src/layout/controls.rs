use leptos::prelude::*;
use sorrow_core::communication::Intent;
use sorrow_core::state::buildings;

use crate::components::{numbers::number_span, Button};
use crate::events::use_keyboard_events;
use crate::state::{use_global_store, BuildingsStoreFields, GlobalStoreStoreFields};

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
            <BuildingButton kind=buildings::Kind::CatnipField />
            // <div>{move || keyboard_events.ctrl.get() }</div>
            // <div>{move || keyboard_events.shift.get() }</div>
            // <div>{move || keyboard_events.alt.get() }</div>
        </div>
    }
}

#[component]
fn BuildingButton(kind: buildings::Kind) -> impl IntoView {
    let store = use_global_store().buildings();
    let level = match kind {
        buildings::Kind::CatnipField => Memo::new(move |_| store.catnip_fields().get()),
    };

    let label = match kind {
        buildings::Kind::CatnipField => "Catnip field",
    };

    view! {
        <Button intent=Intent::Build(kind)>
            {label}" "{number_span(move || level.get())}
        </Button>
    }
}
