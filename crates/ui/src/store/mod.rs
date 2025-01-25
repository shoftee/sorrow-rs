use std::collections::BTreeMap;

use leptos::prelude::*;
use reactive_stores::Store;

use sorrow_core::state::{
    buildings::BuildingKind,
    calendar::SeasonKind,
    precision::Precision,
    recipes::{FulfillmentState, RecipeKind, RECIPE_INGREDIENTS},
    resources::ResourceKind,
    time::RunningState,
    ui::NodeId,
    KeyIter,
};

#[derive(Store)]
pub struct Building {
    pub building: BuildingKind,
    pub level: u32,
}

#[derive(Store)]
pub struct Calendar {
    pub day: i16,
    pub season: SeasonKind,
    pub year: usize,
}

#[derive(Store)]
pub struct Fulfillment {
    pub recipe: RecipeKind,
    pub fulfillment: FulfillmentState,
    pub ingredients: BTreeMap<ResourceKind, Store<IngredientFulfillment>>,
}

#[derive(Store)]
pub struct IngredientFulfillment {
    pub resource: ResourceKind,
    pub required_amount: f64,
}

#[derive(Store)]
pub struct Preferences {
    pub precision: Precision,
}

#[derive(Store)]
pub struct Resource {
    pub resource: ResourceKind,
    pub amount: f64,
    pub delta: f64,
}

#[derive(Store)]
pub struct UiState {
    pub node: NodeId,
    pub visible: bool,
}

#[derive(Store)]
pub struct Global {
    pub is_loaded: bool,

    pub buildings: BTreeMap<BuildingKind, Store<Building>>,
    pub calendar: Calendar,
    pub fulfillments: BTreeMap<RecipeKind, Store<Fulfillment>>,
    pub preferences: Preferences,
    pub resources: BTreeMap<ResourceKind, Store<Resource>>,
    pub running_state: RunningState,
    pub ui: BTreeMap<NodeId, Store<UiState>>,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            is_loaded: false,

            buildings: <BuildingKind as KeyIter>::key_iter()
                .map(|building| (building, Store::new(Building { building, level: 0 })))
                .collect(),
            calendar: Calendar {
                day: 0,
                season: SeasonKind::Spring,
                year: 0,
            },
            fulfillments: <RecipeKind as KeyIter>::key_iter()
                .map(|recipe| {
                    (
                        recipe,
                        Store::new(Fulfillment {
                            recipe,
                            fulfillment: FulfillmentState::Unfulfilled,
                            ingredients: RECIPE_INGREDIENTS
                                .get(&recipe)
                                .expect("Could not find recipe data")
                                .iter()
                                .map(|item| {
                                    (
                                        item.0,
                                        Store::new(IngredientFulfillment {
                                            resource: item.0,
                                            required_amount: item.1,
                                        }),
                                    )
                                })
                                .collect(),
                        }),
                    )
                })
                .collect(),
            preferences: Preferences {
                precision: Precision::default(),
            },
            resources: <ResourceKind as KeyIter>::key_iter()
                .map(|resource| {
                    (
                        resource,
                        Store::new(Resource {
                            resource,
                            amount: 0.0,
                            delta: 0.0,
                        }),
                    )
                })
                .collect(),
            running_state: RunningState::default(),
            ui: <NodeId as KeyIter>::key_iter()
                .map(|node| {
                    (
                        node,
                        Store::new(UiState {
                            node,
                            visible: false,
                        }),
                    )
                })
                .collect(),
        }
    }
}

pub fn provide_store(store: Store<Global>) {
    provide_context(store);
}

pub fn use_global_store() -> Store<Global> {
    expect_context::<Store<Global>>()
}
