use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::{
        BuildChildren, Children, Commands, Component, Event, EventReader, IntoSystemConfigs, Query,
    },
};
use sorrow_core::state::buildings::Kind as BuildingKind;
use sorrow_core::state::recipes::Kind as RecipeKind;
use sorrow_core::state::resources::Kind as ResourceKind;

use crate::{
    index::{IndexedQuery, IndexedQueryMut, LookupIndexPlugin},
    simulation::resources::{Credit, Debit},
};

use super::{
    buildings::Level,
    resources::{Amount, Capacity},
};

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

#[derive(Event)]
pub enum WorkOrder {
    Craft(RecipeKind),
    Construct(BuildingKind),
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Recipe {
    Resource(RecipeKind),
    Building(BuildingKind),
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
struct Ingredient(pub ResourceKind);

#[derive(Component, Debug)]
struct CraftedResource(pub ResourceKind);

#[derive(Default)]
pub struct WorkOrdersPlugin;

impl Plugin for WorkOrdersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookupIndexPlugin::<Recipe>::new())
            .add_event::<WorkOrder>()
            .add_systems(Startup, spawn_recipes)
            .add_systems(FixedUpdate, process_work_orders.in_set(schedule::Main));
    }
}

fn spawn_recipes(mut cmd: Commands) {
    cmd.spawn(Recipe::Resource(RecipeKind::GatherCatnip))
        .with_child((CraftedResource(ResourceKind::Catnip), Amount(1.0)));

    cmd.spawn(Recipe::Resource(RecipeKind::RefineCatnip))
        .with_child((Ingredient(ResourceKind::Catnip), Amount(100.0)))
        .with_child((CraftedResource(ResourceKind::Wood), Amount(1.0)));

    cmd.spawn(Recipe::Building(BuildingKind::CatnipField))
        .with_child((Ingredient(ResourceKind::Catnip), Amount(10.0)));
}

fn process_work_orders(
    mut pending_work_orders: EventReader<WorkOrder>,
    mut resources: IndexedQueryMut<
        super::resources::Kind,
        (&Amount, &mut Debit, &mut Credit, Option<&Capacity>),
    >,
    mut buildings: IndexedQueryMut<super::buildings::Kind, &mut Level>,
    recipes: IndexedQuery<Recipe, &Children>,
    ingredients: Query<(&Ingredient, &Amount)>,
    crafted_resources: Query<(&CraftedResource, &Amount)>,
) {
    let mut deltas = logic::DeltaSetStack::new();
    for (kind, (_, debit, credit, _)) in resources.iter() {
        deltas.add_debit((*kind).into(), (*debit).into());
        deltas.add_credit((*kind).into(), (*credit).into());
    }

    for item in pending_work_orders.read() {
        deltas.push_new();

        let mut is_fulfilled: bool = true;
        match &item {
            WorkOrder::Craft(recipe_kind) => {
                let children = recipes.item(Recipe::Resource(*recipe_kind));
                let ingredients = ingredients.iter_many(children);

                for (kind, amount) in ingredients {
                    deltas.add_credit(kind.0, amount.0);
                    let (amount, debit, credit, _) = resources.item_mut(kind.0.into());
                    let total = amount.0 + debit.0 - credit.0;
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
                    let crafted_resources = crafted_resources.iter_many(children);
                    for (crafted_resource, amount) in crafted_resources {
                        deltas.add_debit(crafted_resource.0, amount.0);
                    }
                }
            }
            WorkOrder::Construct(kind) => {
                let children = recipes.item(Recipe::Building(*kind));

                for (kind, amount) in ingredients.iter_many(children) {
                    deltas.add_credit(kind.0, amount.0);
                    let (amount, debit, credit, _) = resources.item_mut(kind.0.into());
                    let total = amount.0 + debit.0 - credit.0;
                    if total - deltas.credit(kind.0) < 0.0 {
                        is_fulfilled = false;
                        break;
                    }
                }

                if is_fulfilled {
                    let mut level = buildings.item_mut((*kind).into());
                    *level += 1;
                }
            }
        }

        if is_fulfilled {
            deltas.commit();
        } else {
            deltas.roll_back();
        }
    }

    for (kind, logic::ResourceDelta { debit, credit }) in deltas.iter_top() {
        let (_, mut current_debit, mut current_credit, _) = resources.item_mut((*kind).into());
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
