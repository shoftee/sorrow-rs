mod endpoint;
mod index;
mod resources;
mod rpc;
mod runner;
mod work_orders;

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
    use bevy::prelude::IntoSystemSetConfigs;
    use resources::{ResourcesPlugin, ResourcesSystemSet};
    use rpc::{ProcessInputsSystemSet, ProcessOutputsSystemSet, RpcPlugin};
    use runner::TimeoutRunnerPlugin;
    use std::time::Duration;
    use work_orders::{WorkOrdersPlugin, WorkOrdersSystemSet};

    bevy::app::App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin::default())
        .add_plugins(WorkOrdersPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(RpcPlugin)
        .configure_sets(
            Update,
            (
                ProcessInputsSystemSet,
                WorkOrdersSystemSet,
                ResourcesSystemSet,
                ProcessOutputsSystemSet,
            )
                .chain(),
        )
        .run();
}
