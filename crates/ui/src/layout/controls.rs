use leptos::prelude::*;

use sorrow_core::communication::{Intent, WorkOrderKind};
use sorrow_core::state::recipes::Crafting;
use sorrow_core::state::ui::{BonfireNodeId, NodeId};
use sorrow_core::state::{buildings, recipes};

use crate::components::{numbers::number_span, Button};
use crate::state::{
    use_global_store, BuildingStoreFields, FulfillmentStoreFields, GlobalStoreStoreFields,
    UiStateStoreFields,
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
    let bonfire_nodes = [
        (
            NodeId::Bonfire(BonfireNodeId::GatherCatnip),
            WorkOrderKind::Craft(Crafting::GatherCatnip),
        ),
        (
            NodeId::Bonfire(BonfireNodeId::RefineCatnip),
            WorkOrderKind::Craft(Crafting::RefineCatnip),
        ),
        (
            NodeId::Bonfire(BonfireNodeId::CatnipField),
            WorkOrderKind::Construct(buildings::Kind::CatnipField),
        ),
    ];

    let ui = use_global_store().ui();

    let bonfire_buttons = Signal::derive(move || {
        bonfire_nodes
            .iter()
            .filter_map(|(id, kind)| {
                let visible = ui.read_untracked().get(id).unwrap().visible().get();
                if visible {
                    Some(*kind)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="controls grid grid-cols-2 gap-2">
            <For
                each={move || bonfire_buttons.get()}
                key={|state| *state}
                let:item
            >
                <WorkOrderButton kind=item />
            </For>
        </div>
    }
}

#[component]
fn WorkOrderButton(kind: WorkOrderKind) -> impl IntoView {
    let label = match kind {
        WorkOrderKind::Construct(building) => match building {
            sorrow_core::state::buildings::Kind::CatnipField => "Catnip field",
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            sorrow_core::state::recipes::Crafting::GatherCatnip => "Gather catnip",
            sorrow_core::state::recipes::Crafting::RefineCatnip => "Refine catnip",
        },
    };

    let fulfillment = fulfillment_state(kind);
    let is_not_fulfilled = Memo::new(move |_| {
        !matches!(
            fulfillment.get(),
            sorrow_core::state::recipes::Fulfillment::Fulfilled
        )
    });

    match kind {
        WorkOrderKind::Construct(building) => {
            let level = building_level(building);

            view! {
                <Button
                    intent=Intent::QueueWorkOrder(WorkOrderKind::Construct(building))
                    {..}
                    disabled=is_not_fulfilled
                >
                    {label}" "{number_span(level)}
                </Button>
            }
        }
        WorkOrderKind::Craft(crafting) => {
            view! {
                <Button
                    intent=Intent::QueueWorkOrder(WorkOrderKind::Craft(crafting))
                    {..}
                    disabled=is_not_fulfilled
                >
                    {label}
                </Button>
            }
        }
    }
}

fn building_level(kind: sorrow_core::state::buildings::Kind) -> Memo<u32> {
    let buildings = use_global_store().buildings();
    Memo::new(move |_| buildings.read_untracked().get(&kind).unwrap().level().get())
}

fn fulfillment_state(kind: WorkOrderKind) -> Memo<sorrow_core::state::recipes::Fulfillment> {
    let fulfillments = use_global_store().fulfillments();
    let recipe = match kind {
        WorkOrderKind::Craft(crafting) => recipes::Kind::Crafting(crafting),
        WorkOrderKind::Construct(building) => recipes::Kind::Building(building),
    };
    Memo::new(move |_| {
        fulfillments
            .read_untracked()
            .get(&recipe)
            .unwrap()
            .fulfillment()
            .get()
    })
}
