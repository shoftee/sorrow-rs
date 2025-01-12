use std::ops::{AddAssign, SubAssign};

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};
use sorrow_core::communication::Notification;

use super::buildings;
use crate::{
    index::{IndexedQuery, LookupIndexPlugin},
    io::{BufferChanges, OutputEvent},
};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Prepare;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Commit;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Recalculate;
}

pub struct ResourcesPlugin;

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Kind(pub sorrow_core::state::resources::Kind);

impl From<sorrow_core::state::resources::Kind> for Kind {
    fn from(value: sorrow_core::state::resources::Kind) -> Self {
        Self(value)
    }
}

impl From<Kind> for sorrow_core::state::resources::Kind {
    fn from(value: Kind) -> Self {
        value.0
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[require(Debit, Credit)]
pub struct Amount(f64);

impl From<Amount> for f64 {
    fn from(val: Amount) -> Self {
        val.0
    }
}

impl AddAssign<f64> for Amount {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl SubAssign<f64> for Amount {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Capacity(f64);

#[derive(Component, Debug, Clone, Copy)]
pub struct Delta(f64);

impl From<Delta> for f64 {
    fn from(value: Delta) -> Self {
        value.0
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Debit(f64);

impl From<Debit> for f64 {
    fn from(value: Debit) -> Self {
        value.0
    }
}

impl AddAssign<f64> for Debit {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Credit(f64);

impl From<Credit> for f64 {
    fn from(value: Credit) -> Self {
        value.0
    }
}

impl AddAssign<f64> for Credit {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
    }
}

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LookupIndexPlugin::<Kind>::new())
            .add_systems(Startup, spawn_resources)
            .add_systems(
                FixedUpdate,
                (clear_debits_and_credits, add_deltas_to_debit_or_credit)
                    .chain()
                    .in_set(schedule::Prepare),
            )
            .add_systems(
                FixedUpdate,
                commit_credits_and_debits.in_set(schedule::Commit),
            )
            .add_systems(
                FixedPostUpdate,
                recalculate_deltas.in_set(schedule::Recalculate),
            )
            .add_systems(BufferChanges, detect_resource_changes);
    }
}

fn spawn_resources(mut cmd: Commands) {
    for kind in sorrow_core::state::resources::Kind::iter() {
        cmd.spawn((Kind(kind), Amount(0.0), Delta(0.0)));
    }
}

fn clear_debits_and_credits(mut transactions: Query<(&mut Debit, &mut Credit), With<Kind>>) {
    for (mut debit, mut credit) in transactions.iter_mut() {
        debit.0 = 0.0;
        credit.0 = 0.0;
    }
}

fn add_deltas_to_debit_or_credit(
    mut resources: Query<(&Delta, &mut Debit, &mut Credit), With<Kind>>,
) {
    for (delta, mut debit, mut credit) in resources.iter_mut() {
        let delta = delta.0;
        if delta.is_infinite() {
            info!("Encountered infinite delta value");
            continue;
        }
        match delta.signum() {
            -1.0 => *credit += delta,
            1.0 => *debit += delta,
            _ => panic!("Encountered NaN-valued delta value"),
        };
    }
}

fn commit_credits_and_debits(
    mut resources: Query<(&mut Amount, &Debit, &Credit, Option<&Capacity>), With<Kind>>,
) {
    for (mut amount, debit, credit, capacity) in resources.iter_mut() {
        let change = calculate(amount.0, debit.0, credit.0, capacity.map(|f| f.0));
        if let Some(new_amount) = change {
            amount.0 = new_amount;
        }
    }
}

fn calculate(current: f64, debit: f64, credit: f64, capacity: Option<f64>) -> Option<f64> {
    let mut new_amount = current;
    // subtract losses first
    new_amount -= credit;

    let capacity = capacity.unwrap_or(f64::MAX);
    if new_amount < capacity {
        // new resources are gained only when under capacity
        new_amount += debit;

        // but they only go up to capacity at most
        new_amount = f64::min(new_amount, capacity);
    }

    // negative resource amount is non-sense (for now...)
    new_amount = f64::max(new_amount, 0.0);

    // check if the value actually changed
    if (current - new_amount).abs() > f64::EPSILON {
        Some(new_amount)
    } else {
        None
    }
}

fn recalculate_deltas(
    mut resources: Query<(&Kind, &mut Delta)>,
    buildings: IndexedQuery<buildings::Kind, &buildings::Level>,
) {
    for (kind, mut delta) in resources.iter_mut() {
        match kind.0 {
            sorrow_core::state::resources::Kind::Catnip => {
                let catnip_fields =
                    buildings.item(sorrow_core::state::buildings::Kind::CatnipField.into());
                let level: u32 = (*catnip_fields).into();
                delta.0 = 0.125 * level as f64;
            }
            sorrow_core::state::resources::Kind::Wood => {
                // no wood gain yet
            }
        };
    }
}

fn detect_resource_changes(
    resources: Query<(&Kind, Ref<Amount>, Ref<Delta>)>,
    mut outputs: EventWriter<OutputEvent>,
) {
    let mut has_resource_changes = false;
    let mut resource_state = sorrow_core::state::resources::ResourceState::default();
    for (kind, amount, delta) in resources.iter() {
        if amount.is_changed() {
            let amount_state = resource_state.amounts.get_state_mut(&kind.0);
            *amount_state = Some((*amount).into());
            has_resource_changes = true;
        }
        if delta.is_changed() {
            let delta_state = resource_state.deltas.get_state_mut(&kind.0);
            *delta_state = Some((*delta).into());
            has_resource_changes = true;
        }
    }

    if has_resource_changes {
        outputs.send(OutputEvent(Notification::ResourcesChanged(resource_state)));
    }
}
