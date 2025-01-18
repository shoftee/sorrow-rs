use leptos::prelude::*;
use sorrow_core::communication::Intent;
use sorrow_core::state::recipes::Crafting;
use sorrow_core::state::{buildings, recipes};

use crate::components::{numbers::number_span, Button};
use crate::events::use_keyboard_events;
use crate::state::{
    use_global_store, BuildingsStoreFields, FulfillmentStoreFields, GlobalStoreStoreFields,
};

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
            <CraftingButton kind=Crafting::GatherCatnip />
            <CraftingButton kind=Crafting::RefineCatnip />
            <BuildingButton kind=buildings::Kind::CatnipField />
            // <div>{move || keyboard_events.ctrl.get() }</div>
            // <div>{move || keyboard_events.shift.get() }</div>
            // <div>{move || keyboard_events.alt.get() }</div>
        </div>
    }
}

#[component]
fn CraftingButton(kind: Crafting) -> impl IntoView {
    let (intent, label) = match kind {
        Crafting::GatherCatnip => (Intent::GatherCatnip, "Gather catnip"),
        Crafting::RefineCatnip => (Intent::RefineCatnip, "Refine catnip"),
    };

    let fulfillment = fulfillment(recipes::Kind::Crafting(kind));
    let is_not_fulfilled = Memo::new(move |_| {
        !matches!(
            fulfillment.get(),
            sorrow_core::state::recipes::Fulfillment::Fulfilled
        )
    });

    view! {
        <Button intent={intent} {..} disabled=is_not_fulfilled>
            {label}
        </Button>
    }
}

#[component]
fn BuildingButton(kind: buildings::Kind) -> impl IntoView {
    let buildings = use_global_store().buildings();
    let level = match kind {
        buildings::Kind::CatnipField => Memo::new(move |_| buildings.catnip_fields().get()),
    };

    let fulfillment = fulfillment(recipes::Kind::Building(kind));
    let is_not_fulfilled = Memo::new(move |_| {
        !matches!(
            fulfillment.get(),
            sorrow_core::state::recipes::Fulfillment::Fulfilled
        )
    });

    let label = match kind {
        buildings::Kind::CatnipField => "Catnip field",
    };

    view! {
        <Button intent=Intent::Construct(kind) {..} disabled=is_not_fulfilled>
            {label}" "{number_span(move || level.get())}
        </Button>
    }
}

fn fulfillment(kind: recipes::Kind) -> Memo<recipes::Fulfillment> {
    let fulfillments = use_global_store().fulfillments();
    Memo::new(move |_| fulfillments.read().get(&kind).unwrap().fulfillment().get())
}
