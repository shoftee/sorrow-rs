pub mod endpoint;
pub mod worker;

mod dispatcher;
mod world;

use gloo_worker::{Registrable, Spawnable, WorkerBridge};
use sorrow_core::communication::Notification;

use self::worker::Worker;

pub fn register() {
    Worker::registrar().register();
}

pub fn spawn<F>(cb: F, path: &str) -> WorkerBridge<Worker>
where
    F: Fn(Notification) + 'static,
{
    Worker::spawner().callback(cb).spawn(path)
}
