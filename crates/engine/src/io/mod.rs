mod intent_resolver;
mod worker;

use crate::schedules::SchedulesPlugin;

pub use self::worker::Worker;

use bevy::{
    app::{First, Plugin},
    prelude::{Event, IntoSystemSetConfigs},
};

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
            .add_plugins(SchedulesPlugin)
            .add_plugins(WorkerPlugin)
            .add_plugins(IntentResolverPlugin)
            .configure_sets(
                First,
                (worker::sets::Inputs, intent_resolver::sets::Main).chain(),
            );
    }
}
