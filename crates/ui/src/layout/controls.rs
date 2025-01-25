use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::*;

use reactive_stores::Store;
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
        numbers::{number_span, DecimalView, ResourceAmount},
        strings::{ResourceLabel, WorkOrderLabel},
        tooltip::{Target, Tooltip, TooltipContainer},
    },
    endpoint::use_endpoint,
    i18n::use_i18n,
    store::{
        use_global_store, BuildingStoreFields, FulfillmentStoreFields, GlobalStoreFields,
        IngredientFulfillment, IngredientFulfillmentStoreFields, UiStateStoreFields,
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

    let recipe = recipe_for_work_order(kind);

    let fulfillment_state = fulfillment_state(recipe);
    let is_not_fulfilled =
        Memo::new(move |_| !matches!(fulfillment_state.get(), FulfillmentState::Fulfilled));

    let ingredients = ingredients(recipe);

    let button_view = match kind {
        WorkOrderKind::Construct(building) => Either::Left(view! {
            <button
                class="btn w-full" type="button"
                on:click=move |_| endpoint.send(Intent::QueueWorkOrder(WorkOrderKind::Construct(building)))
                prop:disabled=is_not_fulfilled
            >
                <WorkOrderLabel work_order=kind />" "{ number_span(building_level(building)) }
            </button>
        }),
        WorkOrderKind::Craft(crafting) => Either::Right(view! {
            <button
                class="btn w-full" type="button"
                on:click=move |_| endpoint.send(Intent::QueueWorkOrder(WorkOrderKind::Craft(crafting)))
                prop:disabled=is_not_fulfilled
            >
                <WorkOrderLabel work_order=kind />
            </button>
        }),
    };

    view! {
        <TooltipContainer>
            <Target slot>{button_view}</Target>
            <Tooltip slot>
                <div class="flex flex-col *:p-1 rounded p-2 max-w-[20dvw] bg-neutral-100 border border-solid border-neutral-400 drop-shadow-sm">
                    <div>{ button_description(i18n, kind) }</div>
                    <div class="text-sm">
                        <ul>
                            <For
                                each={move || ingredients.get()}
                                key={|item| item.resource().get()}
                                let:item
                            >
                                <IngredientFulfillmentItem store=item />
                            </For>
                        </ul>
                    </div>
                </div>
            </Tooltip>
        </TooltipContainer>
    }
}

#[component]
fn IngredientFulfillmentItem(store: Store<IngredientFulfillment>) -> impl IntoView {
    let kind = store.resource().get_untracked();
    let required_amount = Signal::derive(move || store.required_amount().get());

    view! {
        <li>
            <ResourceLabel resource=kind />
            ": "
            <ResourceAmount resource=kind />
            "/"
            <DecimalView value=required_amount />
        </li>
    }
}

fn building_level(kind: BuildingKind) -> Memo<u32> {
    let buildings = use_global_store().buildings();
    Memo::new(move |_| buildings.read_untracked().get(&kind).unwrap().level().get())
}

fn recipe_for_work_order(kind: WorkOrderKind) -> RecipeKind {
    match kind {
        WorkOrderKind::Craft(crafting) => RecipeKind::Crafting(crafting),
        WorkOrderKind::Construct(building) => RecipeKind::Building(building),
    }
}

fn fulfillment_state(recipe: RecipeKind) -> Memo<FulfillmentState> {
    let store = use_global_store().fulfillments();
    Memo::new(move |_| {
        store
            .read_untracked()
            .get(&recipe)
            .unwrap()
            .fulfillment()
            .get()
    })
}

fn ingredients(recipe: RecipeKind) -> Signal<Vec<Store<IngredientFulfillment>>> {
    let store = use_global_store().fulfillments();
    Signal::derive(move || {
        store
            .read_untracked()
            .get(&recipe)
            .unwrap()
            .ingredients()
            .read_untracked()
            .values()
            .cloned()
            .collect::<Vec<_>>()
    })
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
