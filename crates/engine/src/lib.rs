mod calendar;
mod endpoint;
mod index;
mod io;
mod resources;
mod runner;
mod simulation;
mod work_orders;

pub use endpoint::Endpoint;

pub fn start() {
    use std::time::Duration;

    use bevy::app::{App, FixedUpdate};
    use bevy::log::LogPlugin;
    use bevy::prelude::IntoSystemSetConfigs;

    use calendar::CalendarPlugin;
    use io::InputOutputPlugin;
    use resources::ResourcesPlugin;
    use runner::TimeoutRunnerPlugin;
    use simulation::SimulationPlugin;
    use work_orders::WorkOrdersPlugin;

    App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin::default())
        .add_plugins(SimulationPlugin)
        .add_plugins(CalendarPlugin)
        .add_plugins(WorkOrdersPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(InputOutputPlugin)
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
