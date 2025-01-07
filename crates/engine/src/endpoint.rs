use sorrow_worker::{Spawnable, WorkerBridge};

use sorrow_core::communication::{Intent, Notification};

use super::rpc::Worker;

pub struct Endpoint(WorkerBridge<Worker>);

impl Endpoint {
    pub fn new<F>(cb: F, path: &str) -> Self
    where
        F: 'static + Fn(Notification),
    {
        Self(Worker::spawner().callback(cb).spawn(path))
    }

    pub fn send(&self, command: Intent) {
        self.0.send(command);
    }
}
