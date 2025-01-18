use bevy::{
    app::{First, Plugin},
    prelude::{EventReader, EventWriter, IntoSystemConfigs},
};
use sorrow_core::{
    communication::{Intent, Notification, TimeControl},
    state::{
        recipes::Crafting,
        time::{PartialTimeState, RunningState},
    },
};

use crate::simulation::work_orders::WorkOrder;

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
    mut work_orders: EventWriter<WorkOrder>,
    mut outputs: EventWriter<OutputEvent>,
) {
    for InputEvent(message) in inputs.read() {
        match message {
            Intent::Load => {
                outputs.send(OutputEvent(Notification::Initialized));
            }
            Intent::GatherCatnip => {
                work_orders.send(WorkOrder::Craft(Crafting::GatherCatnip));
            }
            Intent::RefineCatnip => {
                work_orders.send(WorkOrder::Craft(Crafting::RefineCatnip));
            }
            Intent::Construct(kind) => {
                work_orders.send(WorkOrder::Construct(*kind));
            }
            Intent::TimeControl(time_control) => {
                match time_control {
                    TimeControl::Pause => {
                        outputs.send(OutputEvent(Notification::TimeChanged(PartialTimeState {
                            running_state: Some(RunningState::Paused),
                        })));
                    }
                    TimeControl::Start => {
                        outputs.send(OutputEvent(Notification::TimeChanged(PartialTimeState {
                            running_state: Some(RunningState::Running),
                        })));
                    }
                };
            }
        };
    }
}
