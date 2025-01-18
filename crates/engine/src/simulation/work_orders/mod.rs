mod logic;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::*,
};
use sorrow_core::state::{buildings::Kind as BuildingKind, recipes::Crafting as CraftingKind};

use crate::{
    index::{IndexedQuery, IndexedQueryMut},
    simulation::resources::{Credit, Debit},
};

use super::{
    buildings::Level,
    fulfillment::{CraftedAmount, CraftedResource, Ingredient, Recipe, RequiredAmount},
    resources::{Amount, Capacity},
};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Event)]
pub enum WorkOrder {
    Craft(CraftingKind),
    Construct(BuildingKind),
}

pub struct WorkOrdersPlugin;

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOrder>()
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resources: IndexedQueryMut<
        super::resources::Kind,
        (&Amount, &mut Debit, &mut Credit, Option<&Capacity>),
    >,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
    recipes: IndexedQuery<Recipe, &Children>,
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
                let ingredient_entities = recipes.item(Recipe::Craft(*kind));
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
                let ingredient_entities = recipes.item(Recipe::Building(*kind));

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
