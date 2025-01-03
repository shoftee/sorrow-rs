use std::sync::{
    mpsc::{channel, Receiver, Sender},
    LazyLock,
};

use bevy::{
    app::{Plugin, Update},
    prelude::{
        Changed, Event, EventReader, EventWriter, Events, IntoSystemConfigs, IntoSystemSetConfigs,
        NonSend, NonSendMut, Query, ResMut,
    },
};
use gloo_worker::{HandlerId, Worker, WorkerScope};
use sorrow_core::{
    communication::{Intent, Notification, TimeControl},
    state::{
        resources::ResourceState,
        time::{PartialTimeState, RunningState},
        PartialState,
    },
};

use crate::{
    resources::{Amount, Kind},
    work_orders::PendingWorkOrder,
};

enum RemoteEvent {
    Created(WorkerScope<Rpc>),
    Connected(HandlerId),
    Disconnected(HandlerId),
    Received(Intent),
}

struct RemoteEventChannel {
    sender: Sender<RemoteEvent>,
    receiver: Receiver<RemoteEvent>,
}

static mut REMOTE_EVENT_CHANNEL: LazyLock<RemoteEventChannel> = LazyLock::new(|| {
    let (sender, receiver) = channel::<RemoteEvent>();
    RemoteEventChannel { sender, receiver }
});

pub struct Rpc {
    sender: Sender<RemoteEvent>,
}

impl Rpc {
    fn dispatch(&self, event: RemoteEvent) {
        self.sender
            .send(event)
            .expect("Receiver has closed the channel.");
    }
}

impl Worker for Rpc {
    type Message = ();

    type Input = Intent;

    type Output = Notification;

    fn create(scope: &WorkerScope<Self>) -> Self {
        let sender = unsafe {
            #[allow(static_mut_refs)]
            REMOTE_EVENT_CHANNEL.sender.clone()
        };

        let rpc = Self { sender };
        rpc.dispatch(RemoteEvent::Created(scope.clone()));
        rpc
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn connected(&mut self, _: &WorkerScope<Self>, id: HandlerId) {
        self.dispatch(RemoteEvent::Connected(id));
    }

    #[tracing::instrument(level = "trace", fields(id), skip_all)]
    fn disconnected(&mut self, _: &WorkerScope<Self>, id: HandlerId) {
        self.dispatch(RemoteEvent::Disconnected(id));
    }

    #[tracing::instrument(level = "trace", fields(msg), skip_all)]
    fn received(&mut self, _: &WorkerScope<Self>, msg: Self::Input, _: HandlerId) {
        self.dispatch(RemoteEvent::Received(msg));
    }

    fn update(&mut self, _: &WorkerScope<Self>, _: Self::Message) {
        panic!("This worker does not support update().");
    }
}

pub struct Dispatcher {
    scope: Option<WorkerScope<Rpc>>,
    handler_id: Option<HandlerId>,
    inbox: Vec<Intent>,
}

impl Dispatcher {
    fn new() -> Self {
        Self {
            scope: None,
            handler_id: None,
            inbox: Vec::new(),
        }
    }

    fn receive_all(&mut self) {
        unsafe {
            #[allow(static_mut_refs)]
            for event in REMOTE_EVENT_CHANNEL.receiver.try_iter() {
                match event {
                    RemoteEvent::Created(scope) => {
                        debug_assert!(self.scope.is_none(), "Worker is already created");
                        self.scope = Some(scope.clone());
                    }
                    RemoteEvent::Connected(id) => {
                        debug_assert!(self.handler_id.is_none(), "Bridge is already connected");
                        self.handler_id = Some(id);
                    }
                    RemoteEvent::Disconnected(id) => {
                        debug_assert_eq!(
                            self.handler_id.expect("Bridge was already disconnected."),
                            id,
                            "Disconnected bridge had an unexpected ID."
                        );
                        self.handler_id = None;
                    }
                    RemoteEvent::Received(intent) => {
                        self.inbox.push(intent);
                    }
                }
            }
        };
    }

    #[tracing::instrument(level = "trace", fields(notification), skip_all)]
    fn respond(&self, notification: Notification) {
        match (&self.scope, self.handler_id) {
            (Some(scope), Some(id)) => scope.respond(id, notification),
            _ => panic!("Respond was called on an uninitialized dispatcher."),
        }
    }
}

#[derive(Event)]
struct InputEvent(Intent);

#[derive(Event)]
struct OutputEvent(Notification);

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Inputs;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Outputs;
}

pub struct RpcPlugin;

impl Plugin for RpcPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_non_send_resource(Dispatcher::new())
            .add_event::<InputEvent>()
            .add_event::<OutputEvent>()
            .add_systems(
                Update,
                (receive_remote_events, process_inbox, resolve_intents)
                    .chain()
                    .in_set(schedule::Inputs),
            )
            .add_systems(
                Update,
                (detect_changes, send_outputs)
                    .chain()
                    .in_set(schedule::Outputs),
            )
            .configure_sets(Update, schedule::Inputs.before(schedule::Outputs));
    }
}

fn receive_remote_events(mut dispatcher: NonSendMut<Dispatcher>) {
    dispatcher.receive_all();
}

fn process_inbox(mut inputs: EventWriter<InputEvent>, mut dispatcher: NonSendMut<Dispatcher>) {
    let events = dispatcher.inbox.drain(..).map(InputEvent);
    inputs.send_batch(events);
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
                work_orders.send(PendingWorkOrder(
                    crate::work_orders::WorkOrderType::GatherCatnip,
                ));
            }
            Intent::TimeControl(time_control) => {
                let mut time = PartialTimeState::default();
                match time_control {
                    TimeControl::SetAcceleration(a) => {
                        time.acceleration = Some(*a);
                    }
                    TimeControl::Pause => {
                        time.running_state = Some(RunningState::Paused);
                    }
                    TimeControl::Start => {
                        time.running_state = Some(RunningState::Running);
                    }
                };
                outputs.send(OutputEvent(Notification::StateChanged(PartialState {
                    time,
                    ..Default::default()
                })));
            }
            _ => {}
        }
    }
}

fn detect_changes(
    resources: Query<(&Kind, &Amount), Changed<Amount>>,
    mut outputs: EventWriter<OutputEvent>,
) {
    let mut partial_state = ResourceState::default();
    for (kind, &amount) in resources.iter() {
        let state = partial_state.amounts.get_state_mut(&kind.0);
        *state = Some(amount.into());
    }

    outputs.send(OutputEvent(Notification::StateChanged(PartialState {
        resource: partial_state,
        ..Default::default()
    })));
}

fn send_outputs(mut outputs: ResMut<Events<OutputEvent>>, dispatcher: NonSend<Dispatcher>) {
    for OutputEvent(response) in outputs.drain() {
        dispatcher.respond(response);
    }
}
