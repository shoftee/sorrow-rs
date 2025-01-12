use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::{Event, EventReader, IntoSystemConfigs},
};
use sorrow_core::state::buildings;

use crate::{
    index::IndexedQueryMut,
    simulation::resources::{Credit, Debit},
};

use super::buildings::Level;

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Default)]
pub struct WorkOrdersPlugin;

#[derive(Event)]
pub enum WorkOrder {
    Craft(RecipeType),
    Construct(buildings::Kind),
}

pub enum RecipeType {
    GatherCatnip,
    RefineCatnip,
}

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOrder>()
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn process_work_orders(
    mut work_orders: EventReader<WorkOrder>,
    mut resource_tx: IndexedQueryMut<super::resources::Kind, (&mut Debit, &mut Credit)>,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
) {
    use sorrow_core::state::buildings::Kind as BuildingKind;
    use sorrow_core::state::resources::Kind as ResourceKind;

    for work_order in work_orders.read() {
        match &work_order {
            WorkOrder::Craft(recipe) => match recipe {
                RecipeType::GatherCatnip => {
                    let (mut debit, _) = resource_tx.item_mut(ResourceKind::Catnip.into());
                    *debit += 1.0;
                }
                RecipeType::RefineCatnip => {
                    tracing::warn!("Don't know how to refine catnip yet...");
                }
            },
            WorkOrder::Construct(kind) => match kind {
                BuildingKind::CatnipField => {
                    let mut level = buildings.item_mut(BuildingKind::CatnipField.into());
                    *level += 1;
                }
            },
        }
    }
}
