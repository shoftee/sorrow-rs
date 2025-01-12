use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::{Event, EventReader, IntoSystemConfigs},
};
use sorrow_core::state::buildings;

use crate::{
    index::IndexedQueryMut,
    simulation::resources::{Credit, Debit},
};

use super::{buildings::Level, resources::Capacity};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Default)]
pub struct WorkOrdersPlugin;

#[derive(Event)]
pub enum WorkOrder {
    Craft(RecipeType),
    Construct(buildings::Kind),
}

pub enum RecipeType {
    GatherCatnip,
    RefineCatnip,
}

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOrder>()
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resource_tx: IndexedQueryMut<
        super::resources::Kind,
        (&mut Debit, &mut Credit, Option<&Capacity>),
    >,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
) {
    use sorrow_core::state::buildings::Kind as BuildingKind;
    use sorrow_core::state::resources::Kind as ResourceKind;

    let mut deltas = logic::DeltaSetStack::new();
    for (kind, (debit, credit, _)) in resource_tx.iter() {
        deltas.add_debit((*kind).into(), (*debit).into());
        deltas.add_credit((*kind).into(), (*credit).into());
    }

    for item in pending_work_orders.read() {
        deltas.push_new();

        match &item {
            WorkOrder::Craft(recipe) => match recipe {
                RecipeType::GatherCatnip => {
                    deltas.add_debit(ResourceKind::Catnip, 1.0);
                }
                RecipeType::RefineCatnip => {
                    deltas.add_credit(ResourceKind::Catnip, 100.0);
                    deltas.add_debit(ResourceKind::Wood, 1.0);
                }
            },
            WorkOrder::Construct(kind) => match kind {
                BuildingKind::CatnipField => {
                    let mut level = buildings.item_mut(BuildingKind::CatnipField.into());
                    deltas.add_credit(ResourceKind::Catnip, 10.0);
                    *level += 1;
                }
            },
        }

        deltas.commit();
    }

    for (kind, logic::ResourceDelta { debit, credit }) in deltas.iter_top() {
        let (mut current_debit, mut current_credit, _) = resource_tx.item_mut((*kind).into());
        *current_debit += *debit;
        *current_credit += *credit;
    }
}

mod logic {
    use bevy::utils::{hashbrown::hash_map::Iter, HashMap};
    use sorrow_core::state::resources::Kind;

    #[derive(Default)]
    pub struct ResourceDelta {
        pub debit: f64,
        pub credit: f64,
    }

    pub struct DeltaSetStack {
        stack: Vec<HashMap<Kind, ResourceDelta>>,
    }

    impl DeltaSetStack {
        pub fn new() -> Self {
            Self {
                stack: vec![Default::default()],
            }
        }

        pub fn push_new(&mut self) {
            self.stack.push(Default::default());
        }

        #[expect(dead_code)]
        pub fn roll_back(&mut self) {
            assert!(
                self.stack.len() > 1,
                "DeltaSet contains more than one element"
            );
            let _ = self.stack.pop();
        }

        pub fn commit(&mut self) {
            assert!(
                self.stack.len() > 1,
                "DeltaSet contains more than one element"
            );
        }

        #[expect(dead_code)]
        pub fn debit(&self, kind: Kind) -> f64 {
            self.stack
                .iter()
                .rev()
                .filter_map(|ds| ds.get(&kind).map(|l| l.debit))
                .sum()
        }

        #[expect(dead_code)]
        pub fn credit(&self, kind: Kind) -> f64 {
            self.stack
                .iter()
                .rev()
                .filter_map(|ds| ds.get(&kind).map(|l| l.credit))
                .sum()
        }

        fn top(&self) -> &HashMap<Kind, ResourceDelta> {
            self.stack
                .last()
                .expect("DeltaSet contains at least one element")
        }

        pub fn iter_top(&self) -> DeltaSetIter<'_> {
            DeltaSetIter {
                inner: self.top().iter(),
            }
        }

        fn top_mut(&mut self) -> &mut HashMap<Kind, ResourceDelta> {
            self.stack
                .last_mut()
                .expect("DeltaSet contains at least one element")
        }

        pub fn add_debit(&mut self, kind: Kind, amount: f64) {
            let local = self.top_mut().entry(kind).or_default();
            local.debit += amount;
        }

        pub fn add_credit(&mut self, kind: Kind, amount: f64) {
            let local = self.top_mut().entry(kind).or_default();
            local.credit += amount;
        }
    }

    pub struct DeltaSetIter<'a> {
        inner: Iter<'a, Kind, ResourceDelta>,
    }

    impl<'a> Iterator for DeltaSetIter<'a> {
        type Item = (&'a Kind, &'a ResourceDelta);

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }
}
