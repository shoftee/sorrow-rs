use gloo_worker::{Spawnable, WorkerBridge};

use crate::core::communication::{Command, Notification};

use super::worker::Engine;

#[derive(Clone)]
pub struct Endpoint(WorkerBridge<Engine>);

impl Endpoint {
    pub fn new<F>(cb: F, path: &str) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(Engine::spawner().callback(cb).spawn(path))
    }

    pub fn send(&self, command: Command) {
        self.0.send(command);
    }
}
