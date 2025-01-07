mod change_buffer;
mod intent_resolver;
mod worker;

pub use self::worker::Worker;

use bevy::{
    app::{First, Last, Plugin},
    prelude::{Event, IntoSystemSetConfigs},
};

use change_buffer::ChangeBufferPlugin;
use intent_resolver::IntentResolverPlugin;
use sorrow_core::communication::{Intent, Notification};
use worker::WorkerPlugin;

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
            .add_plugins(ChangeBufferPlugin)
            .configure_sets(
                First,
                (worker::schedule::Inputs, intent_resolver::schedule::Main).chain(),
            )
            .configure_sets(
                Last,
                (change_buffer::schedule::Main, worker::schedule::Outputs).chain(),
            );
    }
}
