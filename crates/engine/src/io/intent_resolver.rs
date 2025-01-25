use bevy::{
    app::{First, Plugin},
    prelude::{EventReader, EventWriter, IntoSystemConfigs},
};

use sorrow_core::{
    communication::{EngineMessage, EngineUpdate, Intent, TimeControl},
    state::time::{RunningState, TimeTransport},
};

use crate::simulation::work_orders::WorkOrder;

use super::{InputEvent, OutputEvent, UpdatedEvent};

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct IntentResolverPlugin;

impl Plugin for IntentResolverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(First, resolve_intents.in_set(sets::Main));
    }
}

fn resolve_intents(
    mut inputs: EventReader<InputEvent>,
    mut work_orders: EventWriter<WorkOrder>,
    mut outputs: EventWriter<OutputEvent>,
    mut updates: EventWriter<UpdatedEvent>,
) {
    for InputEvent(message) in inputs.read() {
        match message {
            Intent::Load => {
                outputs.send(OutputEvent(EngineMessage::Loaded));
            }
            Intent::QueueWorkOrder(kind) => {
                work_orders.send(WorkOrder(*kind));
            }
            Intent::TimeControl(time_control) => {
                match time_control {
                    TimeControl::Pause => {
                        updates.send(
                            EngineUpdate::TimeChanged(TimeTransport {
                                running_state: Some(RunningState::Paused),
                            })
                            .into(),
                        );
                    }
                    TimeControl::Start => {
                        updates.send(
                            EngineUpdate::TimeChanged(TimeTransport {
                                running_state: Some(RunningState::Running),
                            })
                            .into(),
                        );
                    }
                };
            }
        };
    }
}
