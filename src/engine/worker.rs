use gloo_worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerScope};
use leptos_reactive::{create_runtime, raw_scope_and_disposer, RuntimeId, Scope};

use crate::core::communication::*;

pub struct Engine {
    handler: CommandHandler,
    runtime: RuntimeId,
    root: Option<Scope>,
}

impl Worker for Engine {
    type Message = ();
    type Input = Command;
    type Output = Notification;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            handler: CommandHandler::new(),
            runtime: create_runtime(),
            root: None,
        }
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, _id: HandlerId) {
        let (root, _) = raw_scope_and_disposer(self.runtime);
        self.root = Some(root);
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        if let Some(notification) = self.handler.handle(msg) {
            scope.respond(id, notification);
        }
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, _id: HandlerId) {
        if let Some(root) = self.root.take() {
            root.dispose();
        }
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, _destruct: WorkerDestroyHandle<Self>) {
        self.runtime.dispose();
    }
}

struct CommandHandler {
    id: u64,
}

impl CommandHandler {
    fn new() -> Self {
        Self { id: 0 }
    }

    fn handle(&mut self, command: Command) -> Option<Notification> {
        Some(match command {
            Command::Initialize => Notification::LogMessage("Worker initialized".to_owned()),
            Command::Increment => {
                self.id += 1;
                Notification::Delta { id: self.id }
            }
        })
    }
}
