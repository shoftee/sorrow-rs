mod visibility;

use bevy::app::Plugin;

use visibility::VisibilityPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(VisibilityPlugin);
    }
}
