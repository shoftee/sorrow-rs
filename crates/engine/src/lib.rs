mod endpoint;
mod index;
mod io;
mod runner;
mod schedules;
mod simulation;
mod ui;

pub use endpoint::Endpoint;
use ui::UiPlugin;

pub fn start() {
    use std::time::Duration;

    use bevy::app::App;
    use bevy::log::LogPlugin;

    use io::InputOutputPlugin;
    use runner::TimeoutRunnerPlugin;
    use simulation::SimulationPlugin;

    App::new()
        .add_plugins(TimeoutRunnerPlugin::new(Duration::from_millis(20)))
        .add_plugins(LogPlugin::default())
        .add_plugins(SimulationPlugin)
        .add_plugins(InputOutputPlugin)
        .add_plugins(UiPlugin)
        .run();
}
