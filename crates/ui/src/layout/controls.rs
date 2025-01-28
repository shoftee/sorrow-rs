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
        strings::ResourceLabel,
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
                key={|kind| *kind}
                let:kind
            >
                <WorkOrderButton kind=kind />
            </For>
        </div>
    }
}

#[component]
fn WorkOrderButton(kind: WorkOrderKind) -> impl IntoView {
    let endpoint = use_endpoint();

    let recipe = recipe_for_work_order(kind);

    let fulfillment_state = fulfillment_state(recipe);
    let is_capped = Memo::new(move |_| matches!(fulfillment_state.get(), FulfillmentState::Capped));
    let is_not_fulfilled =
        Memo::new(move |_| !matches!(fulfillment_state.get(), FulfillmentState::Fulfilled));

    let ingredients = ingredients(recipe);
    let has_ingredients = Memo::new(move |_| ingredients.get().iter().take(1).count() > 0);

    view! {
        <TooltipContainer>
            <Target slot>
                <button
                    type="button"
                    class="btn padded rounded w-full"
                    class:capped=is_capped
                    on:click=move |_| endpoint.send(Intent::QueueWorkOrder(kind))
                    prop:disabled=is_not_fulfilled
                >{
                    match kind {
                        WorkOrderKind::Construct(building_kind) => Either::Left(view! {
                            <Label kind=kind />" "<BuildingLevel building_kind=building_kind />
                        }),
                        WorkOrderKind::Craft(_) => Either::Right(view! {
                            <Label kind=kind />
                        })
                    }
                }</button>
            </Target>
            <Tooltip slot>
                <div class="flex flex-col controls-tooltip-content controls-tooltip-list">
                    <Description kind=kind />
                    <Show when=move || has_ingredients.get()>
                        <ul>
                            <For
                                each={move || ingredients.get()}
                                key={|store| store.resource().get()}
                                let:store
                            >
                                <IngredientFulfillmentItem store=store />
                            </For>
                        </ul>
                    </Show>
                </div>
            </Tooltip>
        </TooltipContainer>
    }
}

#[component]
fn Description(kind: WorkOrderKind) -> impl IntoView {
    let i18n = use_i18n();

    let description = Signal::derive(move || match kind {
        WorkOrderKind::Construct(building) => match building {
            BuildingKind::CatnipField => {
                t_string!(i18n, buildings.catnip_field.description)
            }
        },
        WorkOrderKind::Craft(crafting) => match crafting {
            CraftingRecipeKind::GatherCatnip => {
                t_string!(i18n, bonfire.gather_catnip.description)
            }
            CraftingRecipeKind::RefineCatnip => {
                t_string!(i18n, bonfire.refine_catnip.description)
            }
        },
    });

    view! {
        <p>{ description }</p>
    }
}

#[component]
fn Label(kind: WorkOrderKind) -> impl IntoView {
    let i18n = use_i18n();
    match kind {
        WorkOrderKind::Construct(building_kind) => match building_kind {
            BuildingKind::CatnipField => t_string!(i18n, buildings.catnip_field.label),
        },
        WorkOrderKind::Craft(crafting_recipe_kind) => match crafting_recipe_kind {
            CraftingRecipeKind::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.label),
            CraftingRecipeKind::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.label),
        },
    }
}

#[component]
fn BuildingLevel(building_kind: BuildingKind) -> impl IntoView {
    let buildings = use_global_store().buildings();
    let level = Memo::new(move |_| {
        buildings
            .read_untracked()
            .get(&building_kind)
            .unwrap()
            .level()
            .get()
    });

    number_span(level)
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
