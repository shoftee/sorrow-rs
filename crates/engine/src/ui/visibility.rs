use bevy::app::Plugin;
use bevy::prelude::*;

use sorrow_core::{
    communication::Notification,
    state::ui::{BonfireNodeId, NodeId},
};

use crate::{
    index::{IndexedQueryMut, LookupIndexPlugin},
    io::OutputEvent,
    schedules::{BufferChanges, Recalculate},
    simulation::fulfillment::{Recipe, Unlocked},
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(NodeId);

#[derive(Component, Debug)]
pub enum Visibility {
    Visible,
    Invisible,
}

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LookupIndexPlugin::<Node>::new())
            .add_systems(Startup, spawn_ui_nodes)
            .add_systems(BufferChanges, detect_visibility_changes)
            .add_systems(Recalculate, recalculate_visibility);
    }
}

fn spawn_ui_nodes(mut cmd: Commands) {
    cmd.spawn((
        Node(NodeId::Bonfire(BonfireNodeId::GatherCatnip)),
        Visibility::Visible,
    ));
    cmd.spawn((
        Node(NodeId::Bonfire(BonfireNodeId::RefineCatnip)),
        Visibility::Visible,
    ));
    cmd.spawn((
        Node(NodeId::Bonfire(BonfireNodeId::CatnipField)),
        Visibility::Invisible,
    ));
}

fn recalculate_visibility(
    mut visibilities: IndexedQueryMut<Node, &mut Visibility>,
    recipes: Query<(&Recipe, &Unlocked), Changed<Unlocked>>,
) {
    for (recipe, unlocked) in recipes.iter() {
        let recipe_kind: sorrow_core::state::recipes::Kind = (*recipe).into();
        let node_id: sorrow_core::state::ui::NodeId = recipe_kind.into();

        let mut visibility = visibilities.item_mut(Node(node_id));
        if unlocked.0 {
            *visibility = Visibility::Visible;
        }
    }
}

fn detect_visibility_changes(
    query: Query<(&Node, &Visibility), Changed<Visibility>>,
    mut outputs: EventWriter<OutputEvent>,
) {
    let mut has_changes = false;
    let mut state = sorrow_core::state::ui::VisibilityState::default();
    for (node, visibility) in query.iter() {
        *state.nodes.get_state_mut(&node.0) = match visibility {
            Visibility::Visible => Some(true),
            Visibility::Invisible => Some(false),
        };
        has_changes = true;
    }
    if has_changes {
        outputs.send(OutputEvent(Notification::VisibilityChanged(state)));
    }
}
