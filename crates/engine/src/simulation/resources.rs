use std::ops::{AddAssign, SubAssign};

use bevy::{
    app::{Plugin, Startup},
    prelude::*,
};

use sorrow_core::state::resources::Kind as ResourceKind;
use sorrow_core::{communication::EngineUpdate, state::recipes::Crafting};

use super::{buildings, Unlocked};
use crate::{
    index::{IndexedQuery, LookupIndexPlugin},
    io::UpdatedEvent,
    schedules::BufferChanges,
};

pub mod sets {
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
#[require(Unlocked)]
pub struct Resource(pub ResourceKind);

impl From<ResourceKind> for Resource {
    fn from(value: ResourceKind) -> Self {
        Self(value)
    }
}

impl From<Resource> for ResourceKind {
    fn from(value: Resource) -> Self {
        value.0
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Crafted(pub Crafting);

#[derive(Component, Debug, Clone, Copy)]
#[require(Debit, Credit)]
pub struct Amount(pub f64);

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
pub struct Capacity(pub f64);

#[derive(Component, Debug, Clone, Copy)]
pub struct Delta(f64);

impl From<Delta> for f64 {
    fn from(value: Delta) -> Self {
        value.0
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Debit(pub f64);

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
pub struct Credit(pub f64);

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
        app.add_plugins(LookupIndexPlugin::<Resource>::new())
            .add_systems(Startup, spawn_resources)
            .add_systems(
                FixedUpdate,
                (clear_debits_and_credits, add_deltas_to_debit_or_credit)
                    .chain()
                    .in_set(sets::Prepare),
            )
            .add_systems(FixedUpdate, commit_credits_and_debits.in_set(sets::Commit))
            .add_systems(
                FixedPostUpdate,
                (recalculate_unlocks, recalculate_deltas).in_set(sets::Recalculate),
            )
            .add_systems(BufferChanges, detect_resource_changes);
    }
}

fn spawn_resources(mut cmd: Commands) {
    cmd.spawn((Resource(ResourceKind::Catnip), Amount(0.0), Delta(0.0)));
    cmd.spawn((
        Resource(ResourceKind::Wood),
        Amount(0.0),
        Delta(0.0),
        Crafted(Crafting::RefineCatnip),
    ));
}

fn clear_debits_and_credits(mut transactions: Query<(&mut Debit, &mut Credit), With<Resource>>) {
    for (mut debit, mut credit) in transactions.iter_mut() {
        debit.0 = 0.0;
        credit.0 = 0.0;
    }
}

fn add_deltas_to_debit_or_credit(
    mut resources: Query<(&Delta, &mut Debit, &mut Credit), With<Resource>>,
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
    mut resources: Query<(&mut Amount, &Debit, &Credit, Option<&Capacity>), With<Resource>>,
) {
    for (mut amount, debit, credit, capacity) in resources.iter_mut() {
        if let Some(new_amount) = logic::total_if_changed(&amount, debit, credit, capacity) {
            amount.0 = new_amount;
        }
    }
}

fn recalculate_deltas(
    mut resources: Query<(&Resource, &mut Delta)>,
    buildings: IndexedQuery<buildings::Building, &buildings::Level>,
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

#[expect(clippy::type_complexity)]
fn recalculate_unlocks(
    mut resources: Query<(&Amount, &mut Unlocked), (With<Resource>, Changed<Amount>)>,
) {
    for (amount, mut unlocked) in resources.iter_mut() {
        if amount.0 > 0.0 {
            unlocked.0 = true;
        }
    }
}

fn detect_resource_changes(
    resources: Query<(&Resource, Ref<Amount>, Ref<Delta>)>,
    mut updates: EventWriter<UpdatedEvent>,
) {
    let mut has_changes = false;
    let mut state = sorrow_core::state::resources::ResourceState::default();
    for (kind, amount, delta) in resources.iter() {
        if amount.is_changed() {
            *state.amounts.get_state_mut(&kind.0) = Some(amount.0);
            has_changes = true;
        }
        if delta.is_changed() {
            *state.deltas.get_state_mut(&kind.0) = Some(delta.0);
            has_changes = true;
        }
    }

    if has_changes {
        updates.send(EngineUpdate::ResourcesChanged(state).into());
    }
}

pub mod logic {
    use super::{Amount, Capacity, Credit, Debit};

    pub fn total_if_changed(
        current: &Amount,
        debit: &Debit,
        credit: &Credit,
        capacity: Option<&Capacity>,
    ) -> Option<f64> {
        let new_amount = total(current, debit, credit, capacity);

        // check if the value actually changed
        if (current.0 - new_amount).abs() > f64::EPSILON {
            Some(new_amount)
        } else {
            None
        }
    }

    pub fn total(
        current: &Amount,
        debit: &Debit,
        credit: &Credit,
        capacity: Option<&Capacity>,
    ) -> f64 {
        let capacity = capacity.map(|c| c.0).unwrap_or(f64::MAX);

        let mut new_amount = current.0;
        // subtract losses first
        new_amount -= credit.0;
        if new_amount < capacity {
            // new resources are gained only when under capacity
            new_amount += debit.0;

            // but they only go up to capacity at most
            new_amount = f64::min(new_amount, capacity);
        }

        // negative resource amount is non-sense (for now...)
        new_amount = f64::max(new_amount, 0.0);

        new_amount
    }
}
