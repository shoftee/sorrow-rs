use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::{Event, EventReader, IntoSystemConfigs},
};

use crate::{
    index::IndexedQueryMut,
    simulation::resources::{Credit, Debit, Kind},
};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Default)]
pub struct WorkOrdersPlugin;

pub enum WorkOrderType {
    GatherCatnip,
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
    mut transactions: IndexedQueryMut<Kind, (&mut Debit, &mut Credit)>,
) {
    use sorrow_core::state::resources::Kind as StateKind;

    for work_order in pending_work_orders.read() {
        match work_order.0 {
            WorkOrderType::GatherCatnip => {
                let (mut debit, _) = transactions
                    .get_mut(StateKind::Catnip.into())
                    .expect("Catnip not found :(");
                *debit += 1.0;
            }
        }
    }
}
