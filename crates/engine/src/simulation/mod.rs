pub mod buildings;
pub mod calendar;
pub mod fulfillment;
pub mod resources;
pub mod ticker;
pub mod work_orders;

use bevy::{
    app::{App, FixedPostUpdate, FixedUpdate, Plugin},
    prelude::IntoSystemSetConfigs,
};

use buildings::BuildingsPlugin;
use calendar::CalendarPlugin;
use fulfillment::FulfillmentPlugin;
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
            .add_plugins(FulfillmentPlugin)
            .configure_sets(
                FixedUpdate,
                (
                    ticker::sets::Main,
                    calendar::sets::Main,
                    resources::sets::Prepare,
                    work_orders::sets::Main,
                    resources::sets::Commit,
                )
                    .chain(),
            )
            .configure_sets(
                FixedPostUpdate,
                (resources::sets::Recalculate, fulfillment::sets::Recalculate).chain(),
            );
    }
}
