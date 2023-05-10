use gloo_worker::WorkerBridge;

use crate::core::communication::{Command, Notification};

use super::{engine_spawner, worker::Worker};

#[derive(Clone)]
pub struct Endpoint(WorkerBridge<Worker>);

impl Endpoint {
    pub fn new<F>(cb: F, path: &str) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(engine_spawner().callback(cb).spawn(path))
    }

    pub fn send(&self, command: Command) {
        self.0.send(command);
    }
}
