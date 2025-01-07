use std::sync::{
    mpsc::{channel, Receiver, Sender},
    LazyLock,
};

use sorrow_core::communication::{Intent, Notification};
use sorrow_worker::{HandlerId, WorkerScope};

enum RemoteEvent {
    Created(WorkerScope<Worker>),
    Connected(HandlerId),
    Disconnected(HandlerId),
    Received(Intent),
}

pub struct Dispatcher {
    scope: Option<WorkerScope<Worker>>,
    handler_id: Option<HandlerId>,
    inbox: Vec<Intent>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            scope: None,
            handler_id: None,
            inbox: Vec::new(),
        }
    }

    pub fn drain_inbox(&mut self) -> std::vec::Drain<'_, Intent> {
        self.inbox.drain(..)
    }

    pub fn receive_all(&mut self) {
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
    pub fn respond(&self, notification: Notification) {
        match (&self.scope, self.handler_id) {
            (Some(scope), Some(id)) => scope.respond(id, notification),
            _ => panic!("Respond was called on an uninitialized dispatcher."),
        }
    }
}

struct RemoteEventChannel {
    sender: Sender<RemoteEvent>,
    receiver: Receiver<RemoteEvent>,
}

static mut REMOTE_EVENT_CHANNEL: LazyLock<RemoteEventChannel> = LazyLock::new(|| {
    let (sender, receiver) = channel::<RemoteEvent>();
    RemoteEventChannel { sender, receiver }
});

pub struct Worker {
    sender: Sender<RemoteEvent>,
}

impl Worker {
    fn dispatch(&self, event: RemoteEvent) {
        self.sender
            .send(event)
            .expect("Receiver has closed the channel.");
    }
}

impl sorrow_worker::Worker for Worker {
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
