mod calendar;
mod endpoint;
mod index;
mod resources;
mod rpc;
mod runner;
mod simulation;
mod work_orders;

use bevy::{app::FixedUpdate, prelude::IntoSystemSetConfigs};
use calendar::CalendarPlugin;
pub use endpoint::Endpoint;
use simulation::SimulationPlugin;

pub fn start() {
    run_bevy();
    register();
}

fn register() {
    use gloo_worker::Registrable;
    use rpc::Rpc;

    Rpc::registrar().register();
}

fn run_bevy() {
    use bevy::log::LogPlugin;
    use resources::ResourcesPlugin;
    use rpc::RpcPlugin;
    use runner::TimeoutRunnerPlugin;
    use std::time::Duration;
    use work_orders::WorkOrdersPlugin;

    bevy::app::App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin::default())
        .add_plugins(SimulationPlugin)
        .add_plugins(CalendarPlugin)
        .add_plugins(WorkOrdersPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(RpcPlugin)
        .configure_sets(
            FixedUpdate,
            (
                simulation::schedule::Main,
                calendar::schedule::Main,
                resources::schedule::Prepare,
                work_orders::schedule::Main,
                resources::schedule::Resolve,
            )
                .chain(),
        )
        .run();
}
