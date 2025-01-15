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
