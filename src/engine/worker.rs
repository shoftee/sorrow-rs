use gloo_worker::{HandlerId, WorkerDestroyHandle, WorkerScope};

use crate::core::communication::*;

use super::dispatcher::Dispatcher;

pub struct Worker {
    dispatcher: Dispatcher,
}

impl gloo_worker::Worker for Worker {
    type Message = ();
    type Input = Command;
    type Output = Notification;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            dispatcher: Dispatcher::new(),
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        self.dispatcher.accept(scope.clone(), id, msg);
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, _destruct: WorkerDestroyHandle<Self>) {}
}
