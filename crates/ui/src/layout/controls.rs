use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::*;

use sorrow_core::communication::{Intent, WorkOrderKind};
use sorrow_core::state::recipes::{Crafting, RecipeKind};
use sorrow_core::state::ui::{BonfireNodeId, NodeId};
use sorrow_core::state::{buildings, recipes};

use crate::components::numbers::number_span;
use crate::components::tooltip::{Target, Tooltip, TooltipContainer};
use crate::endpoint::use_endpoint;
use crate::i18n::use_i18n;
use crate::store::{
    use_global_store, BuildingStoreFields, FulfillmentStoreFields, GlobalStoreFields,
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
                if ui.read_untracked().get(id).unwrap().visible().get() {
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
    let i18n = use_i18n();
    let endpoint = use_endpoint();

    let fulfillment = fulfillment_state(kind);
    let is_not_fulfilled =
        Memo::new(move |_| !matches!(fulfillment.get(), recipes::Fulfillment::Fulfilled));

    let button_view = match kind {
        WorkOrderKind::Construct(building) => Either::Left(view! {
            <button
                class="btn w-full" type="button"
                on:click=move |_| endpoint.send(Intent::QueueWorkOrder(WorkOrderKind::Construct(building)))
                prop:disabled=is_not_fulfilled
            >
                { button_label(i18n, kind) }" "{ number_span(building_level(building)) }
            </button>
        }),
        WorkOrderKind::Craft(crafting) => Either::Right(view! {
            <button
                class="btn w-full" type="button"
                on:click=move |_| endpoint.send(Intent::QueueWorkOrder(WorkOrderKind::Craft(crafting)))
                prop:disabled=is_not_fulfilled
            >
                { button_label(i18n, kind) }
            </button>
        }),
    };

    view! {
        <TooltipContainer>
            <Target slot>{button_view}</Target>
            <Tooltip slot>
                <div class="rounded p-2 max-w-[20dvw] bg-neutral-100 border border-solid border-neutral-400 drop-shadow-sm">
                    { button_description(i18n, kind) }
                </div>
            </Tooltip>
        </TooltipContainer>
    }
}

fn building_level(kind: sorrow_core::state::buildings::Kind) -> Memo<u32> {
    let buildings = use_global_store().buildings();
    Memo::new(move |_| buildings.read_untracked().get(&kind).unwrap().level().get())
}

fn fulfillment_state(kind: WorkOrderKind) -> Memo<sorrow_core::state::recipes::Fulfillment> {
    let fulfillments = use_global_store().fulfillments();
    let recipe = match kind {
        WorkOrderKind::Craft(crafting) => RecipeKind::Crafting(crafting),
        WorkOrderKind::Construct(building) => RecipeKind::Building(building),
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

fn button_label(
    i18n: leptos_i18n::I18nContext<crate::i18n::Locale>,
    kind: sorrow_core::communication::WorkOrderKind,
) -> &'static str {
    match kind {
        WorkOrderKind::Construct(building) => match building {
            buildings::Kind::CatnipField => tu_string!(i18n, buildings.catnip_field.label),
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            recipes::Crafting::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.label),
            recipes::Crafting::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.label),
        },
    }
}

fn button_description(
    i18n: leptos_i18n::I18nContext<crate::i18n::Locale>,
    kind: sorrow_core::communication::WorkOrderKind,
) -> impl IntoView {
    let description = match kind {
        WorkOrderKind::Construct(building) => match building {
            buildings::Kind::CatnipField => t_string!(i18n, buildings.catnip_field.description),
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            recipes::Crafting::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.description),
            recipes::Crafting::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.description),
        },
    };
    view! {
        <p>{ description }</p>
    }
}
