use bevy::{
    app::{First, Plugin},
    prelude::{EventReader, EventWriter, IntoSystemConfigs},
};
use sorrow_core::{
    communication::{Intent, Notification, TimeControl},
    state::{
        time::{PartialTimeState, RunningState},
        PartialState,
    },
};
use tracing::warn;

use crate::simulation::work_orders::{PendingWorkOrder, WorkOrderType};

use super::{InputEvent, OutputEvent};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct IntentResolverPlugin;

impl Plugin for IntentResolverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(First, resolve_intents.in_set(schedule::Main));
    }
}

fn resolve_intents(
    mut inputs: EventReader<InputEvent>,
    mut work_orders: EventWriter<PendingWorkOrder>,
    mut outputs: EventWriter<OutputEvent>,
) {
    for InputEvent(message) in inputs.read() {
        match message {
            Intent::Load => {
                outputs.send(OutputEvent(Notification::Initialized));
            }
            Intent::GatherCatnip => {
                work_orders.send(PendingWorkOrder(WorkOrderType::GatherCatnip));
            }
            Intent::TimeControl(time_control) => {
                let mut time = PartialTimeState::default();
                match time_control {
                    TimeControl::Pause => {
                        time.running_state = Some(RunningState::Paused);
                    }
                    TimeControl::Start => {
                        time.running_state = Some(RunningState::Running);
                    }
                };
                outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                    time: Some(time),
                    ..Default::default()
                })));
            }
            unknown => {
                warn!("Received unknown intent: {unknown:?}")
            }
        };
    }
}
