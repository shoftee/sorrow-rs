use gloo_worker::{Spawnable, WorkerBridge};

use sorrow_core::communication::{Intent, Notification};

use super::rpc::Rpc;

pub struct Endpoint(WorkerBridge<Rpc>);

impl Endpoint {
    pub fn new<F>(cb: F, path: &str) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(Rpc::spawner().callback(cb).spawn(path))
    }

    pub fn send(&self, command: Intent) {
        self.0.send(command);
    }
}
