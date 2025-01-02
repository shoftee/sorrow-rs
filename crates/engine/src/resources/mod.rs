use bevy::{
    app::{Plugin, PreUpdate, Startup, Update},
    prelude::{Commands, Component, DetectChangesMut, IntoSystemConfigs, Query, SystemSet, With},
};

use crate::index::LookupIndexPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourcesSystemSet;

pub struct ResourcesPlugin;

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Kind {
    Catnip,
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
            .add_systems(Startup, spawn_resources.in_set(ResourcesSystemSet))
            .add_systems(PreUpdate, clear_deltas.in_set(ResourcesSystemSet))
            .add_systems(Update, resolve_deltas.in_set(ResourcesSystemSet));
    }
}

fn spawn_resources(mut cmd: Commands) {
    cmd.spawn((Kind::Catnip, Amount(0.0), Delta(0.0)));
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
