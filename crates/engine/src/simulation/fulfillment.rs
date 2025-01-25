use bevy::{
    app::{FixedPostUpdate, Plugin, Startup},
    prelude::{
        BuildChildren, Changed, ChildBuild, Children, Commands, Component, EventWriter,
        IntoSystemConfigs, ParamSet, Query, With,
    },
    utils::HashMap,
};

use sorrow_core::{
    communication::{EngineUpdate, FulfillmentTransport},
    state::{
        buildings::{BUILDING_PRICE_RATIOS, BUILDING_UNLOCK_RATIOS},
        recipes::{
            FulfillmentState, RecipeKind, ResourceAmount, RECIPE_CRAFTED_RESOURCES,
            RECIPE_INGREDIENTS,
        },
        resources::ResourceKind,
        KeyIter,
    },
};

use crate::{
    index::{IndexedQuery, LookupIndexPlugin},
    io::UpdatedEvent,
    schedules::BufferChanges,
    simulation::resources::Capacity,
};

use super::{
    buildings::{Building, Level},
    resources::{Amount, Crafted, Resource},
};

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Recalculate;
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[require(Fulfillment)]
pub struct Recipe(pub RecipeKind);

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
#[require(super::Unlocked)]
struct UnlockRatio(pub f64);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fulfillment(pub FulfillmentState);

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
    for recipe in RecipeKind::key_iter() {
        let mut spawned = cmd.spawn(Recipe(recipe));

        match recipe {
            RecipeKind::Crafting(crafting_recipe_kind) => {
                let ResourceAmount(resource, base_amount) = RECIPE_CRAFTED_RESOURCES
                    .get(&crafting_recipe_kind)
                    .expect("recipe did not have a crafting result entry");
                spawned.with_child((CraftedResource(*resource), CraftedAmount(*base_amount)));
            }
            RecipeKind::Building(building_kind) => {
                let price_ratio = BUILDING_PRICE_RATIOS
                    .get(&building_kind)
                    .expect("building recipe did not have a price ratio entry");
                spawned.insert(PriceRatio(*price_ratio));

                if let Some(unlock_ratio) = BUILDING_UNLOCK_RATIOS.get(&building_kind) {
                    spawned.insert(UnlockRatio(*unlock_ratio));
                }
            }
        }

        spawned.with_children(|b| {
            for ResourceAmount(resource, base_amount) in RECIPE_INGREDIENTS
                .get(&recipe)
                .expect("recipe did not have an ingredients entry")
            {
                b.spawn((
                    Ingredient(*resource),
                    BaseAmount(*base_amount),
                    RequiredAmount(*base_amount),
                ));
            }
        });
    }
}

fn recalculate_recipe_costs(
    buildings: Query<(&Building, &Level), Changed<Level>>,
    recipes: IndexedQuery<Recipe, (&PriceRatio, &Children)>,
    mut amounts_query: Query<(&mut RequiredAmount, &BaseAmount), With<Ingredient>>,
) {
    for (building, level) in buildings.iter() {
        let (ratio, ingredient_entities) = recipes.item(Recipe(RecipeKind::Building(building.0)));
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
    resources: IndexedQuery<Resource, (&Amount, Option<&Capacity>, Option<&Crafted>)>,
) {
    fn recalculate_one(
        recipe: Recipe,
        calculated: &mut HashMap<Recipe, FulfillmentState>,
        recipes: &IndexedQuery<Recipe, &Children>,
        requirements: &Query<(&Ingredient, &RequiredAmount), With<Ingredient>>,
        resources: &IndexedQuery<Resource, (&Amount, Option<&Capacity>, Option<&Crafted>)>,
    ) -> FulfillmentState {
        let mut result = FulfillmentState::Fulfilled;
        let children = recipes.item(recipe);
        for (ingredient, required_amount) in requirements.iter_many(children) {
            let (amount, capacity, crafted) = resources.item(ingredient.0.into());
            if let Some(crafted) = crafted {
                result = recalculate_one(
                    Recipe(RecipeKind::Crafting(crafted.0)),
                    calculated,
                    recipes,
                    requirements,
                    resources,
                );
                break;
            }
            if let Some(capacity) = capacity {
                if required_amount.0 > capacity.0 {
                    result = FulfillmentState::Capped;
                    break;
                }
            }
            if amount.0 < required_amount.0 {
                result = FulfillmentState::Unfulfilled;
                break;
            }
        }
        calculated.insert(recipe, result);
        result
    }

    let mut calculated = HashMap::<Recipe, FulfillmentState>::default();

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
    mut recipes: Query<(&UnlockRatio, &mut super::Unlocked, &Children), With<Recipe>>,
    requirements: Query<(&Ingredient, &RequiredAmount), With<Ingredient>>,
    resources: IndexedQuery<Resource, &Amount>,
) {
    for (unlock_ratio, mut unlocked, children) in recipes.iter_mut() {
        for (ingredient, required_amount) in requirements.iter_many(children) {
            let amount = resources.item(ingredient.0.into());
            if amount.0 >= (required_amount.0 * unlock_ratio.0) {
                unlocked.0 = true;
                break;
            }
        }
    }
}

fn detect_fulfillment_changes(
    fulfillments: Query<(&Recipe, &Fulfillment), Changed<Fulfillment>>,
    mut updates: EventWriter<UpdatedEvent>,
) {
    let mut has_changes = false;
    let mut transport = FulfillmentTransport::default();
    for (recipe, fulfillment) in fulfillments.iter() {
        *transport.fulfillments.get_state_mut(&recipe.0) = Some(fulfillment.0);
        has_changes = true;
    }

    if has_changes {
        updates.send(EngineUpdate::FulfillmentsChanged(transport).into());
    }
}
