use gloo_worker::{Spawnable, WorkerBridge};

use crate::core::communication::{Command, Notification};

use super::worker::Engine;

#[derive(Clone)]
pub struct EngineEndpoint(WorkerBridge<Engine>);

impl EngineEndpoint {
    pub fn new<F>(cb: F) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(Engine::spawner().callback(cb).spawn("./engine.js"))
    }

    pub fn send(&self, command: Command) {
        self.0.send(command);
    }
}
