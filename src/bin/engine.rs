use gloo_worker::Registrable;
use sorrow::engine::worker::Engine;

fn main() {
    Engine::registrar().register();
}
