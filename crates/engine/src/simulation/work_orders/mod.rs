mod logic;

use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::{
        BuildChildren, Children, Commands, Component, Event, EventReader, IntoSystemConfigs, Query,
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
}

#[derive(Event)]
pub enum WorkOrder {
    Craft(RecipeKind),
    Construct(BuildingKind),
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Recipe {
    Resource(RecipeKind),
    Building(BuildingKind),
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
struct Ingredient(pub ResourceKind);

#[derive(Component, Debug)]
struct CraftedResource(pub ResourceKind);

#[derive(Component, Debug, Copy, Clone)]
struct PriceRatio(pub f64);

#[derive(Default)]
pub struct WorkOrdersPlugin;

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookupIndexPlugin::<Recipe>::new())
            .add_event::<WorkOrder>()
            .add_systems(Startup, spawn_recipes)
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn spawn_recipes(mut cmd: Commands) {
    cmd.spawn(Recipe::Resource(RecipeKind::GatherCatnip))
        .with_child((CraftedResource(ResourceKind::Catnip), Amount(1.0)));

    cmd.spawn(Recipe::Resource(RecipeKind::RefineCatnip))
        .with_child((Ingredient(ResourceKind::Catnip), Amount(100.0)))
        .with_child((CraftedResource(ResourceKind::Wood), Amount(1.0)));

    cmd.spawn((
        Recipe::Building(BuildingKind::CatnipField),
        PriceRatio(1.12),
    ))
    .with_child((Ingredient(ResourceKind::Catnip), Amount(10.0)));
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resources: IndexedQueryMut<
        super::resources::Kind,
        (&Amount, &mut Debit, &mut Credit, Option<&Capacity>),
    >,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
    recipes: IndexedQuery<Recipe, (Option<&PriceRatio>, &Children)>,
    ingredients: Query<(&Ingredient, &Amount)>,
    crafted_resources: Query<(&CraftedResource, &Amount)>,
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
            WorkOrder::Craft(recipe_kind) => {
                let (_, children) = recipes.item(Recipe::Resource(*recipe_kind));
                let ingredients = ingredients.iter_many(children);

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
                    let crafted_resources = crafted_resources.iter_many(children);
                    for (crafted_resource, amount) in crafted_resources {
                        deltas.add_debit(crafted_resource.0, amount.0);
                    }
                }
            }
            WorkOrder::Construct(kind) => {
                let mut level = buildings.item_mut((*kind).into());

                let (price_ratio, children) = recipes.item(Recipe::Building(*kind));
                let price_ratio = price_ratio.map(|r| r.0).unwrap_or(1.0);

                for (kind, amount) in ingredients.iter_many(children) {
                    let adjusted_for_level = amount.0 * price_ratio.powi(level.0 as i32);
                    deltas.add_credit(kind.0, adjusted_for_level);

                    let (amount, debit, credit, _) = resources.item_mut(kind.0.into());
                    let total = amount.0 + debit.0 - credit.0;
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
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
