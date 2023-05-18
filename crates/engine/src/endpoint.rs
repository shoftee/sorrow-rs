use gloo_worker::WorkerBridge;

use sorrow_core::communication::{Command, Notification};

use super::{spawn, worker::Worker};

#[derive(Clone)]
pub struct Endpoint(WorkerBridge<Worker>);

impl Endpoint {
    pub fn new<F>(cb: F, path: &str) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(spawn(cb, path))
    }

    pub fn send(&self, command: Command) {
        self.0.send(command);
    }
}
