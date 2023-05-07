use sorrow::engine::endpoint::EngineEndpoint;
use sorrow::ui::app;

use sorrow::core::communication::{Command, Notification};

fn main() {
    let endpoint = EngineEndpoint::new(|n: Notification| match n {
        Notification::LogMessage(msg) => leptos::log!("Received message: {}", msg),
        Notification::Delta { id } => leptos::log!("Received delta: {}", id),
    });
    endpoint.send(Command::Initialize);
    app::mount(endpoint);
}
