use bevy::app::{FixedPostUpdate, Plugin, Startup};
use bevy::prelude::*;
use bevy::utils::HashMap;

use sorrow_core::communication::Notification;
use sorrow_core::state::buildings::Kind as BuildingKind;
use sorrow_core::state::recipes::Kind as RecipeKind;
use sorrow_core::state::recipes::{Fulfillment as SFulfillment, FulfillmentState};
use sorrow_core::state::resources::Kind as ResourceKind;

use crate::index::{IndexedQuery, LookupIndexPlugin};
use crate::io::OutputEvent;
use crate::schedules::BufferChanges;
use crate::simulation::resources::Capacity;

use super::buildings::Level;
use super::resources::{Amount, Crafted};

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Recalculate;
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[require(Fulfillment)]
pub enum Recipe {
    Building(BuildingKind),
    Craft(sorrow_core::state::recipes::Crafting),
}

impl From<RecipeKind> for Recipe {
    fn from(value: RecipeKind) -> Self {
        match value {
            RecipeKind::Building(kind) => Recipe::Building(kind),
            RecipeKind::Crafting(kind) => Recipe::Craft(kind),
        }
    }
}

impl From<Recipe> for RecipeKind {
    fn from(value: Recipe) -> Self {
        match value {
            Recipe::Craft(kind) => RecipeKind::Crafting(kind),
            Recipe::Building(kind) => RecipeKind::Building(kind),
        }
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ingredient(pub ResourceKind);

#[derive(Component, Debug, Default)]
pub struct RequiredAmount(pub f64);

#[derive(Component, Debug)]
struct BaseAmount(pub f64);

#[derive(Component, Debug)]
pub struct CraftedResource(pub ResourceKind);

#[derive(Component, Debug)]
pub struct CraftedAmount(pub f64);

#[derive(Component, Debug, Copy, Clone)]
struct PriceRatio(pub f64);

#[derive(Component, Debug, Copy, Clone)]
#[require(Unlocked)]
struct UnlockRatio(pub f64);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fulfillment(pub SFulfillment);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unlocked(pub bool);

pub struct FulfillmentPlugin;

impl Plugin for FulfillmentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LookupIndexPlugin::<Recipe>::new())
            .add_systems(Startup, spawn_recipes)
            .add_systems(
                FixedPostUpdate,
                (
                    recalculate_recipe_costs,
                    recalculate_fulfillments,
                    recalculate_unlocks,
                )
                    .chain()
                    .in_set(sets::Recalculate),
            )
            .add_systems(BufferChanges, detect_fulfillment_changes);
    }
}

fn spawn_recipes(mut cmd: Commands) {
    cmd.spawn(Recipe::Craft(
        sorrow_core::state::recipes::Crafting::GatherCatnip,
    ))
    .with_child((CraftedResource(ResourceKind::Catnip), CraftedAmount(1.0)));

    cmd.spawn(Recipe::Craft(
        sorrow_core::state::recipes::Crafting::RefineCatnip,
    ))
    .with_child((
        Ingredient(ResourceKind::Catnip),
        BaseAmount(100.0),
        RequiredAmount(100.0),
    ))
    .with_child((CraftedResource(ResourceKind::Wood), CraftedAmount(1.0)));

    cmd.spawn((
        Recipe::Building(BuildingKind::CatnipField),
        PriceRatio(1.12),
        UnlockRatio(0.3),
    ))
    .with_child((
        Ingredient(ResourceKind::Catnip),
        BaseAmount(10.0),
        RequiredAmount(10.0),
    ));
}

fn recalculate_recipe_costs(
    buildings: Query<(&super::buildings::Kind, &Level), Changed<Level>>,
    recipes: IndexedQuery<Recipe, (&PriceRatio, &Children)>,
    mut amounts_query: Query<(&mut RequiredAmount, &BaseAmount), With<Ingredient>>,
) {
    for (building, level) in buildings.iter() {
        let (ratio, ingredient_entities) = recipes.item(Recipe::Building(building.0));
        let mut amounts = amounts_query.iter_many_mut(ingredient_entities);
        while let Some((mut required_amount, base_amount)) = amounts.fetch_next() {
            required_amount.0 = base_amount.0 * (ratio.0.powi(level.0 as i32));
        }
    }
}

#[expect(clippy::type_complexity)]
fn recalculate_fulfillments(
    mut recipes: ParamSet<(
        IndexedQuery<Recipe, &Children>,
        Query<(&Recipe, &mut Fulfillment)>,
    )>,
    requirements: Query<(&Ingredient, &RequiredAmount), With<Ingredient>>,
    resources: IndexedQuery<super::resources::Kind, (&Amount, Option<&Capacity>, Option<&Crafted>)>,
) {
    fn recalculate_one(
        recipe: Recipe,
        calculated: &mut HashMap<Recipe, SFulfillment>,
        recipes: &IndexedQuery<Recipe, &Children>,
        requirements: &Query<(&Ingredient, &RequiredAmount), With<Ingredient>>,
        resources: &IndexedQuery<
            super::resources::Kind,
            (&Amount, Option<&Capacity>, Option<&Crafted>),
        >,
    ) -> SFulfillment {
        let mut result = SFulfillment::Fulfilled;
        let children = recipes.item(recipe);
        for (ingredient, required_amount) in requirements.iter_many(children) {
            let (amount, capacity, crafted) = resources.item(ingredient.0.into());
            if let Some(crafted) = crafted {
                result = recalculate_one(
                    Recipe::Craft(crafted.0),
                    calculated,
                    recipes,
                    requirements,
                    resources,
                );
                break;
            }
            if let Some(capacity) = capacity {
                if required_amount.0 > capacity.0 {
                    result = SFulfillment::Capped;
                    break;
                }
            }
            if amount.0 < required_amount.0 {
                result = SFulfillment::Unfulfilled;
                break;
            }
        }
        calculated.insert(recipe, result);
        result
    }

    let mut calculated = HashMap::<Recipe, SFulfillment>::default();

    let recipes_indexed = recipes.p0();
    for recipe in recipes_indexed.keys() {
        recalculate_one(
            *recipe,
            &mut calculated,
            &recipes_indexed,
            &requirements,
            &resources,
        );
    }

    let mut recipes_mutable = recipes.p1();
    for (recipe, mut fulfillment) in recipes_mutable.iter_mut() {
        fulfillment.0 = *calculated.get(recipe).unwrap();
    }
}

fn recalculate_unlocks(
    mut recipes: Query<(&UnlockRatio, &mut Unlocked, &Children), With<Recipe>>,
    requirements: Query<(&Ingredient, &RequiredAmount), With<Ingredient>>,
    resources: IndexedQuery<super::resources::Kind, &Amount>,
) {
    for (unlock_ratio, mut fulfillment, children) in recipes.iter_mut() {
        for (ingredient, required_amount) in requirements.iter_many(children) {
            let amount = resources.item(ingredient.0.into());
            if amount.0 >= (required_amount.0 * unlock_ratio.0) {
                fulfillment.0 = true;
                break;
            }
        }
    }
}

fn detect_fulfillment_changes(
    fulfillments: Query<(&Recipe, &Fulfillment), Changed<Fulfillment>>,
    mut outputs: EventWriter<OutputEvent>,
) {
    let mut has_changes = false;
    let mut state = FulfillmentState::default();
    for (recipe, fulfillment) in fulfillments.iter() {
        let recipe = (*recipe).into();
        *state.fulfillments.get_state_mut(&recipe) = Some(fulfillment.0);
        has_changes = true;
    }

    if has_changes {
        outputs.send(OutputEvent(Notification::FulfillmentsChanged(state)));
    }
}
