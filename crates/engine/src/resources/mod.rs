use bevy::{
    app::{Plugin, Startup, Update},
    prelude::{Commands, Component, DetectChangesMut, IntoSystemConfigs, Query, With},
};
use sorrow_core::state::resources::Kind as StateKind;
use strum::IntoEnumIterator;

use crate::index::LookupIndexPlugin;

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Prepare;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Resolve;
}

pub struct ResourcesPlugin;

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Kind(pub StateKind);

impl From<StateKind> for Kind {
    fn from(value: StateKind) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Amount(f64);

impl From<Amount> for f64 {
    fn from(val: Amount) -> Self {
        val.0
    }
}

#[derive(Component, Debug)]
pub struct Delta(f64);

impl std::ops::AddAssign<f64> for Delta {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl std::ops::SubAssign<f64> for Delta {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LookupIndexPlugin::<Kind>::new())
            .add_systems(Startup, spawn_resources)
            .add_systems(Update, clear_deltas.in_set(schedule::Prepare))
            .add_systems(Update, resolve_deltas.in_set(schedule::Resolve));
    }
}

fn spawn_resources(mut cmd: Commands) {
    for kind in <StateKind as IntoEnumIterator>::iter() {
        cmd.spawn((Kind(kind), Amount(0.0), Delta(0.0)));
    }
}

fn clear_deltas(mut deltas: Query<&mut Delta, With<Kind>>) {
    for mut delta in deltas.iter_mut() {
        delta.0 = 0.0;
    }
}

fn resolve_deltas(mut resources: Query<(&mut Amount, &Delta), With<Kind>>) {
    for (mut amount, delta) in resources.iter_mut() {
        amount.0 += delta.0;
        amount.set_changed();
    }
}
