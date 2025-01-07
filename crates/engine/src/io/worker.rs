use std::sync::mpsc::{channel, Receiver, SendError, Sender, TryRecvError};

use bevy::{
    app::{First, Last, Plugin},
    prelude::{EventWriter, Events, IntoSystemConfigs, NonSend, ResMut},
};
use sorrow_core::communication::{Intent, Notification};
use sorrow_worker::{HandlerId, Registrable, WorkerScope};
use tracing::warn;

use super::{InputEvent, OutputEvent};

#[derive(Clone)]
pub struct Dispatcher {
    notification_channel: Sender<Notification>,
    intent_channel: Sender<Intent>,
}

impl Dispatcher {
    #[tracing::instrument(level = "trace", fields(notification), skip_all)]
    pub fn respond(&self, notification: Notification) {
        match self.notification_channel.send(notification) {
            Ok(_) => {}
            Err(error) => panic!("Could not send notification to UI: {error}"),
        }
    }
}

pub struct Worker {
    scope: WorkerScope<Worker>,
    handler_id: Option<HandlerId>,
}

impl Worker {
    fn dispatcher(&self) -> &Dispatcher {
        self.scope.external_state()
    }
}

impl sorrow_worker::Worker for Worker {
    type ExternalState = Dispatcher;

    type Message = ();

    type Input = Intent;

    type Output = Notification;

    fn create(scope: &WorkerScope<Self>) -> Self {
        Self {
            scope: scope.clone(),
            handler_id: None,
        }
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn connected(&mut self, _: &WorkerScope<Self>, id: HandlerId) {
        self.handler_id = Some(id);
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn disconnected(&mut self, _: &WorkerScope<Self>, _: HandlerId) {
        self.handler_id = None;
    }

    #[tracing::instrument(level = "trace", fields(msg), skip_all)]
    fn received(&mut self, _: &WorkerScope<Self>, msg: Self::Input, _: HandlerId) {
        if let Err(SendError(_)) = self.dispatcher().intent_channel.send(msg) {
            warn!("Could not send intent to backend");
        }
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {
        panic!("This worker does not support update().");
    }
}

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Inputs;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Outputs;
}

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let (notification_sender, notification_receiver) = channel();
        let (intent_sender, intent_receiver) = channel();
        Worker::registrar().register_with(Dispatcher {
            notification_channel: notification_sender.clone(),
            intent_channel: intent_sender.clone(),
        });

        app.insert_non_send_resource(notification_receiver)
            .insert_non_send_resource(intent_receiver)
            .add_systems(First, receive_inputs.in_set(schedule::Inputs))
            .add_systems(Last, send_outputs.in_set(schedule::Outputs));
    }
}

fn receive_inputs(mut inputs: EventWriter<InputEvent>, intent_receiver: NonSend<Receiver<Intent>>) {
    loop {
        match intent_receiver.try_recv() {
            Ok(item) => inputs.send(InputEvent(item)),
            Err(TryRecvError::Empty) => break,
            Err(e) => panic!("Could not receive further inputs: {e}"),
        };
    }
}

fn send_outputs(mut outputs: ResMut<Events<OutputEvent>>, dispatcher: NonSend<Dispatcher>) {
    for OutputEvent(response) in outputs.drain() {
        dispatcher.respond(response);
    }
}
