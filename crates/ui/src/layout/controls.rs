use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::*;

use sorrow_core::{
    communication::{Intent, WorkOrderKind},
    state::{
        buildings::BuildingKind,
        recipes::{CraftingRecipeKind, FulfillmentState, RecipeKind},
        ui::{BonfireNodeId, NodeId},
    },
};

use crate::{
    components::{
        numbers::number_span,
        tooltip::{Target, Tooltip, TooltipContainer},
    },
    endpoint::use_endpoint,
    i18n::use_i18n,
    store::{
        use_global_store, BuildingStoreFields, FulfillmentStoreFields, GlobalStoreFields,
        UiStateStoreFields,
    },
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
            WorkOrderKind::Craft(CraftingRecipeKind::GatherCatnip),
        ),
        (
            NodeId::Bonfire(BonfireNodeId::RefineCatnip),
            WorkOrderKind::Craft(CraftingRecipeKind::RefineCatnip),
        ),
        (
            NodeId::Bonfire(BonfireNodeId::CatnipField),
            WorkOrderKind::Construct(BuildingKind::CatnipField),
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
        Memo::new(move |_| !matches!(fulfillment.get(), FulfillmentState::Fulfilled));

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

fn building_level(kind: BuildingKind) -> Memo<u32> {
    let buildings = use_global_store().buildings();
    Memo::new(move |_| buildings.read_untracked().get(&kind).unwrap().level().get())
}

fn fulfillment_state(kind: WorkOrderKind) -> Memo<FulfillmentState> {
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
    kind: WorkOrderKind,
) -> &'static str {
    match kind {
        WorkOrderKind::Construct(building) => match building {
            BuildingKind::CatnipField => tu_string!(i18n, buildings.catnip_field.label),
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            CraftingRecipeKind::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.label),
            CraftingRecipeKind::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.label),
        },
    }
}

fn button_description(
    i18n: leptos_i18n::I18nContext<crate::i18n::Locale>,
    kind: WorkOrderKind,
) -> impl IntoView {
    let description = match kind {
        WorkOrderKind::Construct(building) => match building {
            BuildingKind::CatnipField => {
                t_string!(i18n, buildings.catnip_field.description)
            }
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            CraftingRecipeKind::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.description),
            CraftingRecipeKind::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.description),
        },
    };
    view! {
        <p>{ description }</p>
    }
}
