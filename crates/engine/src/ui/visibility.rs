use bevy::{
    app::{Plugin, Startup},
    prelude::{Changed, Commands, Component, EventWriter, Query},
};

use sorrow_core::{
    communication::{EngineUpdate, VisibilityTransport},
    state::ui::{NodeId, NODE_VISIBILITY},
};

use crate::{
    index::{IndexedQueryMut, LookupIndexPlugin},
    io::UpdatedEvent,
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
    cmd.spawn_batch(NODE_VISIBILITY.iter().map(|(id, is_visible)| {
        (
            Node(*id),
            if *is_visible {
                Visibility::Visible
            } else {
                Visibility::Invisible
            },
        )
    }));
}

fn recalculate_visibility(
    mut visibilities: IndexedQueryMut<Node, &mut Visibility>,
    recipes: Query<(&Recipe, &Unlocked), Changed<Unlocked>>,
    resources: Query<(&Resource, &Unlocked), Changed<Unlocked>>,
) {
    let recipe_states = recipes.iter().map(|(recipe, unlocked)| {
        let node_id: NodeId = recipe.0.into();
        (Node(node_id), unlocked)
    });

    let resource_states = resources.iter().map(|(resource, unlocked)| {
        let node_id: NodeId = resource.0.into();
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
    mut updates: EventWriter<UpdatedEvent>,
) {
    let mut has_changes = false;
    let mut transport = VisibilityTransport::default();
    for (node, visibility) in query.iter() {
        *transport.nodes.get_state_mut(&node.0) = match visibility {
            Visibility::Visible => Some(true),
            Visibility::Invisible => Some(false),
        };
        has_changes = true;
    }

    if has_changes {
        updates.send(EngineUpdate::VisibilityChanged(transport).into());
    }
}
