pub mod buildings;
pub mod calendar;
pub mod resources;
pub mod ticker;
pub mod work_orders;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::IntoSystemSetConfigs,
};

use buildings::BuildingsPlugin;
use calendar::CalendarPlugin;
use resources::ResourcesPlugin;
use ticker::TickerPlugin;
use work_orders::WorkOrdersPlugin;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TickerPlugin)
            .add_plugins(CalendarPlugin)
            .add_plugins(WorkOrdersPlugin)
            .add_plugins(ResourcesPlugin)
            .add_plugins(BuildingsPlugin)
            .configure_sets(
                FixedUpdate,
                (
                    ticker::schedule::Main,
                    calendar::schedule::Main,
                    resources::schedule::Prepare,
                    work_orders::schedule::Main,
                    resources::schedule::Resolve,
                )
                    .chain(),
            );
    }
}
