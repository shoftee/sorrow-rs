mod logic;

use bevy::{
    app::{App, FixedPostUpdate, FixedUpdate, Plugin, Startup},
    prelude::{
        BuildChildren, Changed, Children, Commands, Component, Event, EventReader,
        IntoSystemConfigs, Query, With,
    },
};
use sorrow_core::state::buildings::Kind as BuildingKind;
use sorrow_core::state::recipes::Kind as RecipeKind;
use sorrow_core::state::resources::Kind as ResourceKind;

use crate::{
    index::{IndexedQuery, IndexedQueryMut, LookupIndexPlugin},
    simulation::resources::{Credit, Debit},
};

use super::{
    buildings::Level,
    resources::{Amount, Capacity},
};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Recalculate;
}

#[derive(Event)]
pub enum WorkOrder {
    Craft(RecipeKind),
    Construct(BuildingKind),
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CraftingRecipe(pub RecipeKind);

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct BuildingRecipe(pub BuildingKind);

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
struct Ingredient(pub ResourceKind);

#[derive(Component, Debug)]
struct RequiredAmount(pub f64);

#[derive(Component, Debug)]
struct BaseAmount(pub f64);

#[derive(Component, Debug)]
struct CraftedResource(pub ResourceKind);

#[derive(Component, Debug)]
struct CraftedAmount(pub f64);

#[derive(Component, Debug, Copy, Clone)]
struct PriceRatio(pub f64);

#[derive(Default)]
pub struct WorkOrdersPlugin;

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOrder>()
            .add_plugins(LookupIndexPlugin::<CraftingRecipe>::new())
            .add_plugins(LookupIndexPlugin::<BuildingRecipe>::new())
            .add_systems(Startup, spawn_recipes)
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main))
            .add_systems(
                FixedPostUpdate,
                recalculate_recipe_costs.in_set(schedule::Recalculate),
            );
    }
}

fn spawn_recipes(mut cmd: Commands) {
    cmd.spawn(CraftingRecipe(RecipeKind::GatherCatnip))
        .with_child((CraftedResource(ResourceKind::Catnip), CraftedAmount(1.0)));

    cmd.spawn(CraftingRecipe(RecipeKind::RefineCatnip))
        .with_child((
            Ingredient(ResourceKind::Catnip),
            RequiredAmount(100.0),
            BaseAmount(100.0),
        ))
        .with_child((CraftedResource(ResourceKind::Wood), CraftedAmount(1.0)));

    cmd.spawn((BuildingRecipe(BuildingKind::CatnipField), PriceRatio(1.12)))
        .with_child((
            Ingredient(ResourceKind::Catnip),
            RequiredAmount(10.0),
            BaseAmount(10.0),
        ));
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resources: IndexedQueryMut<
        super::resources::Kind,
        (&Amount, &mut Debit, &mut Credit, Option<&Capacity>),
    >,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
    crafting_recipes: IndexedQuery<CraftingRecipe, &Children>,
    building_recipes: IndexedQuery<BuildingRecipe, &Children>,
    ingredients: Query<(&Ingredient, &RequiredAmount)>,
    crafted_resources: Query<(&CraftedResource, &CraftedAmount)>,
) {
    let mut deltas = logic::DeltaSetStack::new();
    for (kind, (_, debit, credit, _)) in resources.iter() {
        deltas.add_debit((*kind).into(), (*debit).into());
        deltas.add_credit((*kind).into(), (*credit).into());
    }

    for item in pending_work_orders.read() {
        deltas.push_new();

        let mut is_fulfilled: bool = true;
        match &item {
            WorkOrder::Craft(kind) => {
                let ingredient_entities = crafting_recipes.item(CraftingRecipe(*kind));
                let ingredients = ingredients.iter_many(ingredient_entities);

                for (kind, amount) in ingredients {
                    deltas.add_credit(kind.0, amount.0);

                    let (amount, debit, credit, _) = resources.item_mut(kind.0.into());
                    let total = amount.0 + debit.0 - credit.0;
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
                    let crafted_resources = crafted_resources.iter_many(ingredient_entities);
                    for (crafted_resource, amount) in crafted_resources {
                        deltas.add_debit(crafted_resource.0, amount.0);
                    }
                }
            }
            WorkOrder::Construct(kind) => {
                let ingredient_entities = building_recipes.item(BuildingRecipe(*kind));

                for (kind, amount) in ingredients.iter_many(ingredient_entities) {
                    deltas.add_credit(kind.0, amount.0);

                    let (amount, debit, credit, _) = resources.item_mut(kind.0.into());
                    let total = amount.0 + debit.0 - credit.0;
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
                    let mut level = buildings.item_mut((*kind).into());
                    *level += 1;
                }
            }
        }

        if is_fulfilled {
            deltas.commit();
        } else {
            deltas.roll_back();
        }
    }

    for (kind, logic::ResourceDelta { debit, credit }) in deltas.iter_top() {
        let (_, mut current_debit, mut current_credit, _) = resources.item_mut((*kind).into());
        *current_debit += *debit;
        *current_credit += *credit;
    }
}

fn recalculate_recipe_costs(
    buildings: Query<(&super::buildings::Kind, &Level), Changed<Level>>,
    building_recipes: IndexedQuery<BuildingRecipe, (&PriceRatio, &Children)>,
    mut amounts_query: Query<(&mut RequiredAmount, &BaseAmount), With<Ingredient>>,
) {
    for (building, level) in buildings.iter() {
        let (ratio, ingredient_entities) = building_recipes.item(BuildingRecipe(building.0));
        let mut amounts = amounts_query.iter_many_mut(ingredient_entities);
        while let Some((mut required_amount, base_amount)) = amounts.fetch_next() {
            required_amount.0 = base_amount.0 * (ratio.0.powi(level.0 as i32));
        }
    }
}
