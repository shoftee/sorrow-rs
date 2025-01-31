mod logic;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::{Children, Event, EventReader, IntoSystemConfigs, Query},
};

use sorrow_core::{communication::WorkOrderKind, state::recipes::RecipeKind};

use crate::{
    index::{IndexedQuery, IndexedQueryMut},
    simulation::resources::{Credit, Debit},
};

use super::{
    buildings::{Building, Level},
    fulfillment::{CraftedAmount, CraftedResource, Ingredient, Recipe, RequiredAmount},
    resources::{self, Amount, Capacity, Resource},
};

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Event)]
pub struct WorkOrder(pub WorkOrderKind);

pub struct WorkOrdersPlugin;

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOrder>()
            .add_systems(FixedUpdate, process_work_orders.in_set(sets::Main));
    }
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resources: IndexedQueryMut<Resource, (&Amount, &mut Debit, &mut Credit, Option<&Capacity>)>,
    mut buildings: IndexedQueryMut<Building, &mut Level>,
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
        match &item.0 {
            WorkOrderKind::Craft(crafting) => {
                let ingredient_entities = recipes.item(Recipe(RecipeKind::Crafting(*crafting)));
                let ingredients = ingredients.iter_many(ingredient_entities);

                for (kind, amount) in ingredients {
                    deltas.add_credit(kind.0, amount.0);

                    let (amount, debit, credit, capacity) = resources.item_mut(kind.0.into());
                    let total = resources::logic::total(amount, &debit, &credit, capacity);
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
            WorkOrderKind::Construct(building) => {
                let ingredient_entities = recipes.item(Recipe(RecipeKind::Building(*building)));

                for (kind, amount) in ingredients.iter_many(ingredient_entities) {
                    deltas.add_credit(kind.0, amount.0);

                    let (amount, debit, credit, capacity) = resources.item_mut(kind.0.into());
                    let total = resources::logic::total(amount, &debit, &credit, capacity);
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
                    let mut level = buildings.item_mut((*building).into());
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

    // Overwriting here is correct because the delta set includes the original debit and credit values.
    for (kind, logic::ResourceDelta { debit, credit }) in deltas.collect() {
        let (_, mut current_debit, mut current_credit, _) = resources.item_mut(kind.into());
        current_debit.0 = debit;
        current_credit.0 = credit;
    }
}
