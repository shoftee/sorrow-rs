use std::{default::Default, time::Duration};

use sorrow_core::{
    communication::*,
    state::{PartialResourceState, RunningState},
    utils::Shared,
};

use bevy::{
    app::{App, PostUpdate, PreUpdate, Update},
    log::*,
    prelude::{Event, EventReader, EventWriter, Events, NonSend, ResMut},
};
use gloo_worker::{HandlerId, WorkerScope};

use crate::runner::TimeoutRunnerPlugin;

pub struct Worker {
    dispatcher: Shared<Dispatcher>,
}

struct Dispatcher {
    scope_info: Option<(HandlerId, WorkerScope<Worker>)>,
    received_inputs: Vec<Command>,
}

impl Dispatcher {
    fn new() -> Self {
        Self {
            scope_info: None,
            received_inputs: Vec::new(),
        }
    }

    fn respond(&self, output: Notification) {
        if let Some((id, scope)) = &self.scope_info {
            scope.respond(*id, output);
        } else {
            warn!("scope_info is missing, can't send responses to UI.")
        }
    }
}

#[derive(Event)]
struct InputEvent(<Worker as gloo_worker::Worker>::Input);

#[derive(Event)]
struct OutputEvent(<Worker as gloo_worker::Worker>::Output);

impl gloo_worker::Worker for Worker {
    type Message = ();
    type Input = Command;
    type Output = Notification;

    fn create(_: &WorkerScope<Self>) -> Self {
        let dispatcher = Shared::new(Dispatcher::new());
        run_app(dispatcher.clone());
        Self { dispatcher }
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {}

    fn connected(&mut self, scope: &WorkerScope<Self>, id: HandlerId) {
        debug!("Connected handler {:?}", id);

        self.dispatcher.borrow_mut().scope_info = Some((id, scope.clone()));
    }

    fn disconnected(&mut self, _: &WorkerScope<Self>, id: HandlerId) {
        debug!("Disconnected handler {:?}", id);

        self.dispatcher.borrow_mut().scope_info = None;
    }

    fn received(&mut self, _: &WorkerScope<Self>, input: Self::Input, _: HandlerId) {
        debug!("Received input {:?}", input);

        self.dispatcher.borrow_mut().received_inputs.push(input);
    }
}

fn run_app(dispatcher: Shared<Dispatcher>) {
    App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin {
            level: Level::INFO,
            ..Default::default()
        })
        .insert_non_send_resource(dispatcher)
        .add_event::<InputEvent>()
        .add_event::<OutputEvent>()
        .add_systems(PreUpdate, receive_inputs)
        .add_systems(Update, main_step)
        .add_systems(PostUpdate, send_outputs)
        .run();
}

fn receive_inputs(mut inputs: EventWriter<InputEvent>, dispatcher: NonSend<Shared<Dispatcher>>) {
    let mut dispatcher = dispatcher.borrow_mut();
    let events = dispatcher.received_inputs.drain(..).map(InputEvent);
    inputs.send_batch(events);
}

fn main_step(mut inputs: EventReader<InputEvent>, mut outputs: EventWriter<OutputEvent>) {
    for InputEvent(message) in inputs.read() {
        match message {
            Command::Load => {
                outputs.send(OutputEvent(Notification::Initialized));
            }
            Command::GatherCatnip => {
                outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                    resource: Some(PartialResourceState { catnip: Some(1.0) }),
                    ..Default::default()
                })));
            }
            Command::TimeControl(time_control) => match time_control {
                TimeControl::SetAcceleration(a) => {
                    outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                        acceleration: Some(*a),
                        ..Default::default()
                    })));
                }
                TimeControl::Pause => {
                    outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                        running_state: Some(RunningState::Paused),
                        ..Default::default()
                    })));
                }
                TimeControl::Start => {
                    outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                        running_state: Some(RunningState::Running),
                        ..Default::default()
                    })));
                }
            },
            _ => {}
        }
    }
}

fn send_outputs(mut outputs: ResMut<Events<OutputEvent>>, dispatcher: NonSend<Shared<Dispatcher>>) {
    for OutputEvent(response) in outputs.drain() {
        dispatcher.borrow().respond(response);
    }
}
