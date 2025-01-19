use std::ops::AddAssign;

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
use sorrow_core::{
    communication::Notification,
    state::buildings::{BuildingState, Kind as BuildingKind},
};

use crate::{index::LookupIndexPlugin, io::OutputEvent, schedules::BufferChanges};

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Building(pub BuildingKind);

impl From<BuildingKind> for Building {
    fn from(value: BuildingKind) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Level(pub u32);

impl From<Level> for u32 {
    fn from(value: Level) -> Self {
        value.0
    }
}

impl AddAssign<u32> for Level {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookupIndexPlugin::<Building>::new())
            .add_systems(Startup, spawn_buildings)
            .add_systems(BufferChanges, detect_building_changes);
    }
}

fn spawn_buildings(mut commands: Commands) {
    commands.spawn((Building(BuildingKind::CatnipField), Level(0)));
}

fn detect_building_changes(
    buildings: Query<(&Building, &Level), Changed<Level>>,
    mut outputs: EventWriter<OutputEvent>,
) {
    let mut has_building_changes = false;
    let mut building_state = BuildingState::default();
    for (kind, level) in buildings.iter() {
        let level_state = building_state.levels.get_state_mut(&kind.0);
        *level_state = Some(level.0);
        has_building_changes = true;
    }

    if has_building_changes {
        outputs.send(OutputEvent(Notification::BuildingsChanged(building_state)));
    }
}
