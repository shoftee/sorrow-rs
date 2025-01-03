mod endpoint;
mod index;
mod resources;
mod rpc;
mod runner;
mod work_orders;

use bevy::prelude::IntoSystemSetConfigs;
pub use endpoint::Endpoint;

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
    use bevy::app::Update;
    use bevy::log::LogPlugin;
    use resources::ResourcesPlugin;
    use rpc::RpcPlugin;
    use runner::TimeoutRunnerPlugin;
    use std::time::Duration;
    use work_orders::WorkOrdersPlugin;

    bevy::app::App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin::default())
        .add_plugins(WorkOrdersPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(RpcPlugin)
        .configure_sets(
            Update,
            (
                resources::schedule::Prepare,
                rpc::schedule::Inputs,
                work_orders::schedule::Main,
                resources::schedule::Resolve,
                rpc::schedule::Outputs,
            )
                .chain(),
        )
        .run();
}
