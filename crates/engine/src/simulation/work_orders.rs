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

pub enum WorkOrderType {
    GatherCatnip,
    Build(buildings::Kind),
}

#[derive(Event)]
pub struct PendingWorkOrder(pub WorkOrderType);

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PendingWorkOrder>()
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn process_work_orders(
    mut pending_work_orders: EventReader<PendingWorkOrder>,
    mut resource_tx: IndexedQueryMut<super::resources::Kind, (&mut Debit, &mut Credit)>,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
) {
    use sorrow_core::state::buildings::Kind as BuildingKind;
    use sorrow_core::state::resources::Kind as ResourceKind;

    for item in pending_work_orders.read() {
        match item.0 {
            WorkOrderType::GatherCatnip => {
                let (mut debit, _) = resource_tx.get_mut(ResourceKind::Catnip.into()).unwrap();
                *debit += 1.0;
            }
            WorkOrderType::Build(kind) => match kind {
                BuildingKind::CatnipField => {
                    let mut level = buildings.get_mut(BuildingKind::CatnipField.into()).unwrap();
                    *level += 1;
                }
            },
        }
    }
}
