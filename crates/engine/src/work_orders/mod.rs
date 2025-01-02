use bevy::{
    log::error,
    prelude::{Event, EventReader, IntoSystemConfigs, SystemSet},
};

use crate::{
    index::IndexedQueryMut,
    resources::{Delta, Kind},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]

pub struct WorkOrdersSystemSet;

#[derive(Default)]
pub struct WorkOrdersPlugin;

pub enum WorkOrderType {
    GatherCatnip,
}

#[derive(Event)]
pub struct PendingWorkOrder(pub WorkOrderType);

impl bevy::app::Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PendingWorkOrder>().add_systems(
            bevy::app::Update,
            process_work_orders.in_set(WorkOrdersSystemSet),
        );
    }
}

fn process_work_orders(
    mut pending_work_orders: EventReader<PendingWorkOrder>,
    mut deltas: IndexedQueryMut<Kind, &mut Delta>,
) {
    for work_order in pending_work_orders.read() {
        match work_order.0 {
            WorkOrderType::GatherCatnip => match deltas.get_mut(Kind::Catnip) {
                Ok(ref mut delta) => {
                    **delta += 1.0;
                }
                Err(err) => {
                    error!("Couldn't get catnip data: {:?}", err);
                }
            },
        }
    }
}
