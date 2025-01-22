use bevy::{
    app::{First, Last, Plugin},
    prelude::{EventWriter, Events, IntoSystemConfigs, NonSend, ResMut},
};
use sorrow_core::{
    communication::{EngineMessage, Intent},
    utils::Shared,
};
use sorrow_worker::{HandlerId, Registrable, WorkerDestroyHandle, WorkerScope};

use super::{InputEvent, OutputEvent, UpdatedEvent};

pub struct Dispatcher {
    inputs: Vec<Intent>,
    outputs: Vec<EngineMessage>,
    scope: Option<WorkerScope<Worker>>,
    handler_id: Option<HandlerId>,
}

impl Dispatcher {
    fn created(&mut self, scope: WorkerScope<Worker>) {
        self.scope = Some(scope.clone());
    }

    fn connected(&mut self, id: HandlerId) {
        self.handler_id = Some(id);
    }

    fn disconnected(&mut self) {
        self.handler_id = None;
    }

    fn received(&mut self, msg: Intent) {
        self.inputs.push(msg)
    }

    fn destroyed(&mut self) {
        self.scope = None;
    }

    fn send_responses(&mut self) {
        if let (Some(scope), Some(handler_id)) = (self.scope.clone(), self.handler_id) {
            for message in self.outputs.drain(..) {
                scope.respond(handler_id, message);
            }
        } else {
            panic!("Could not send responses because there was no connection");
        }
    }
}

pub struct Worker {
    scope: WorkerScope<Worker>,
}

impl Worker {
    fn dispatcher(&self) -> &Shared<Dispatcher> {
        self.scope.external_state()
    }
}

impl sorrow_worker::Worker for Worker {
    type ExternalState = Shared<Dispatcher>;

    type Message = ();

    type Input = Intent;

    type Output = EngineMessage;

    fn create(scope: &WorkerScope<Self>) -> Self {
        scope.external_state().borrow_mut().created(scope.clone());
        Self {
            scope: scope.clone(),
        }
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn connected(&mut self, _: &WorkerScope<Self>, id: HandlerId) {
        self.dispatcher().borrow_mut().connected(id);
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn disconnected(&mut self, _: &WorkerScope<Self>, _: HandlerId) {
        self.dispatcher().borrow_mut().disconnected();
    }

    #[tracing::instrument(level = "trace", fields(msg), skip_all)]
    fn received(&mut self, _: &WorkerScope<Self>, msg: Self::Input, _: HandlerId) {
        self.dispatcher().borrow_mut().received(msg);
    }

    fn destroy(&mut self, _: &WorkerScope<Self>, _: WorkerDestroyHandle<Self>) {
        self.dispatcher().borrow_mut().destroyed();
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {
        panic!("This worker does not support update()");
    }
}

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Inputs;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Outputs;
}

pub struct WorkerPlugin;

impl Plugin for WorkerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let dispatcher = Shared::new(Dispatcher {
            inputs: Vec::<Intent>::new(),
            outputs: Vec::<EngineMessage>::new(),
            handler_id: None,
            scope: None,
        });
        Worker::registrar().register_with(dispatcher.clone());

        app.insert_non_send_resource(dispatcher)
            .add_systems(First, receive_inputs.in_set(sets::Inputs))
            .add_systems(
                Last,
                (batch_updates, send_outputs).chain().in_set(sets::Outputs),
            );
    }
}

fn receive_inputs(mut inputs: EventWriter<InputEvent>, dispatcher: NonSend<Shared<Dispatcher>>) {
    inputs.send_batch(dispatcher.borrow_mut().inputs.drain(..).map(InputEvent));
}

fn batch_updates(mut updates: ResMut<Events<UpdatedEvent>>, mut outputs: EventWriter<OutputEvent>) {
    outputs.send(EngineMessage::Updated(updates.drain().map(|e| e.0).collect()).into());
}

fn send_outputs(mut outputs: ResMut<Events<OutputEvent>>, dispatcher: NonSend<Shared<Dispatcher>>) {
    let mut dispatcher = dispatcher.borrow_mut();
    dispatcher.outputs.extend(outputs.drain().map(|e| e.0));
    dispatcher.send_responses();
}
