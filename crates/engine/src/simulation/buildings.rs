use std::ops::AddAssign;

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
use sorrow_core::state::buildings::Kind as StateKind;

use crate::index::LookupIndexPlugin;

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Kind(pub StateKind);

impl From<StateKind> for Kind {
    fn from(value: StateKind) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Level(u32);

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
        app.add_plugins(LookupIndexPlugin::<Kind>::new())
            .add_systems(Startup, spawn_buildings);
    }
}

fn spawn_buildings(mut commands: Commands) {
    commands.spawn((Kind(StateKind::CatnipField), Level(0)));
}
