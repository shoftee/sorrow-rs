pub mod endpoint;
pub mod worker;

pub fn start() {
    use gloo_worker::Registrable;

    worker::Engine::registrar().register();
}
