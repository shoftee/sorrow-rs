mod intent_resolver;
mod worker;

pub use self::worker::Worker;

use bevy::{
    app::{First, Last, MainScheduleOrder, Plugin},
    ecs::schedule::ScheduleLabel,
    prelude::{Event, IntoSystemSetConfigs},
};

use intent_resolver::IntentResolverPlugin;
use sorrow_core::communication::{Intent, Notification};
use worker::WorkerPlugin;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferChanges;

#[derive(Event)]
pub struct InputEvent(pub Intent);

#[derive(Event)]
pub struct OutputEvent(pub Notification);

pub struct InputOutputPlugin;

impl Plugin for InputOutputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<InputEvent>()
            .add_event::<OutputEvent>()
            .add_plugins(WorkerPlugin)
            .add_plugins(IntentResolverPlugin)
            .configure_sets(
                First,
                (worker::schedule::Inputs, intent_resolver::schedule::Main).chain(),
            );

        app.init_schedule(BufferChanges);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(Last, BufferChanges);
    }
}
