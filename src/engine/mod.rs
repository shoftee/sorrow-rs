pub mod endpoint;
pub mod worker;

mod controller;
mod world;

use gloo_worker::{Registrable, Spawnable, WorkerRegistrar, WorkerSpawner};

use self::worker::Worker;

pub fn engine_registrar() -> WorkerRegistrar<Worker> {
    Worker::registrar()
}

pub fn engine_spawner() -> WorkerSpawner<Worker> {
    Worker::spawner()
}
