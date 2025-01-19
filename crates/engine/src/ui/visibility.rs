use bevy::{
    app::{Plugin, Startup},
    prelude::{Changed, Commands, Component, EventWriter, Query},
};

use sorrow_core::{
    communication::Notification,
    state::{
        ui::{BonfireNodeId, NodeId},
        KeyIter,
    },
};

use crate::{
    index::{IndexedQueryMut, LookupIndexPlugin},
    io::OutputEvent,
    schedules::{BufferChanges, Recalculate},
    simulation::{fulfillment::Recipe, resources::Resource, Unlocked},
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
            .add_systems(Recalculate, recalculate_visibility)
            .add_systems(BufferChanges, detect_visibility_changes);
    }
}

fn spawn_ui_nodes(mut cmd: Commands) {
    let bundles = NodeId::key_iter().map(|node_id| {
        (
            Node(node_id),
            if matches!(
                node_id,
                NodeId::Bonfire(BonfireNodeId::GatherCatnip)
                    | NodeId::Bonfire(BonfireNodeId::RefineCatnip)
            ) {
                Visibility::Visible
            } else {
                Visibility::Invisible
            },
        )
    });
    cmd.spawn_batch(bundles);
}

fn recalculate_visibility(
    mut visibilities: IndexedQueryMut<Node, &mut Visibility>,
    recipes: Query<(&Recipe, &Unlocked), Changed<Unlocked>>,
    resources: Query<(&Resource, &Unlocked), Changed<Unlocked>>,
) {
    let recipe_states = recipes.iter().map(|(recipe, unlocked)| {
        let node_id: sorrow_core::state::ui::NodeId = recipe.0.into();
        (Node(node_id), unlocked)
    });

    let resource_states = resources.iter().map(|(resource, unlocked)| {
        let node_id: sorrow_core::state::ui::NodeId = resource.0.into();
        (Node(node_id), unlocked)
    });

    for (node, unlocked) in recipe_states.chain(resource_states) {
        if unlocked.0 {
            *visibilities.item_mut(node) = Visibility::Visible;
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
