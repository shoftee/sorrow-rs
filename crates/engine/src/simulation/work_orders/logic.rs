use bevy::utils::{hashbrown::hash_map::Iter, HashMap};
use sorrow_core::state::resources::ResourceKind;

#[derive(Debug, Default, Clone, Copy)]
pub struct ResourceDelta {
    pub debit: f64,
    pub credit: f64,
}

#[derive(Debug)]
pub struct DeltaSetStack {
    stack: Vec<HashMap<ResourceKind, ResourceDelta>>,
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
    pub fn debit(&self, kind: ResourceKind) -> f64 {
        self.stack
            .iter()
            .rev()
            .filter_map(|ds| ds.get(&kind).map(|l| l.debit))
            .sum()
    }

    pub fn credit(&self, kind: ResourceKind) -> f64 {
        self.stack
            .iter()
            .rev()
            .filter_map(|ds| ds.get(&kind).map(|l| l.credit))
            .sum()
    }

    pub fn collect(&self) -> Vec<(ResourceKind, ResourceDelta)> {
        let mut keyed = HashMap::<ResourceKind, ResourceDelta>::default();
        for level in self.stack.iter() {
            for (resource, delta) in level.iter() {
                let entry = keyed.entry(*resource).or_insert(Default::default());
                entry.debit += delta.debit;
                entry.credit += delta.credit;
            }
        }
        keyed.into_iter().collect()
    }

    fn top_mut(&mut self) -> &mut HashMap<ResourceKind, ResourceDelta> {
        self.stack
            .last_mut()
            .expect("DeltaSet contains at least one element")
    }

    pub fn add_debit(&mut self, kind: ResourceKind, amount: f64) {
        let local = self.top_mut().entry(kind).or_default();
        local.debit += amount;
    }

    pub fn add_credit(&mut self, kind: ResourceKind, amount: f64) {
        let local = self.top_mut().entry(kind).or_default();
        local.credit += amount;
    }
}

pub struct DeltaSetIter<'a> {
    inner: Iter<'a, ResourceKind, ResourceDelta>,
}

impl<'a> Iterator for DeltaSetIter<'a> {
    type Item = (&'a ResourceKind, &'a ResourceDelta);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
