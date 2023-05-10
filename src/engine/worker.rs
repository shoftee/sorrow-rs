use gloo_worker::{HandlerId, WorkerDestroyHandle, WorkerScope};

use crate::core::communication::*;

use super::controller::Controller;

pub struct Worker {
    controller: Controller,
}

impl gloo_worker::Worker for Worker {
    type Message = ();
    type Input = Command;
    type Output = Notification;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            controller: Controller::new(),
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        self.controller.accept(scope.clone(), id, msg);
    }

    fn destroy(&mut self, _scope: &WorkerScope<Self>, _destruct: WorkerDestroyHandle<Self>) {}
}
