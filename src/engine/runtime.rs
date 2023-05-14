use leptos_reactive::{
    IntoSignalSetter, RuntimeId, RwSignal, Scope, Signal, SignalGet, SignalSetter, SignalUpdate,
    SignalWith,
};
use sorrow_reactive::{CreateState, CreateStateSlice, Get, Set, Update, With};

pub struct State<T>(RwSignal<T>)
where
    T: 'static;

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for State<T> {}

impl<T> With<T> for State<T> {
    fn with<Output>(&self, f: impl FnOnce(&T) -> Output) -> Output {
        SignalWith::with(&self.0, f)
    }
}

impl<T> Update<T> for State<T> {
    fn update(&self, f: impl FnOnce(&mut T)) {
        SignalUpdate::update(&self.0, f);
    }
}

pub struct StateSlice<T>(Signal<T>, SignalSetter<T>)
where
    T: Clone + 'static;

impl<T> Clone for StateSlice<T>
where
    T: Clone + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for StateSlice<T> where T: Clone + 'static {}

impl<T: Clone> Get<T> for StateSlice<T>
where
    T: 'static,
{
    fn get(&self) -> T {
        Signal::get(&self.0)
    }
}

impl<T: Clone> Set<T> for StateSlice<T> {
    fn set(&self, new_value: T) {
        self.1.set(new_value);
    }
}

pub trait CreateReactive<Source> {
    type Target;

    fn create_reactive(&self, value: Source) -> Self::Target;
}

pub struct Runtime {
    runtime_id: RuntimeId,
    scope: Scope,
}

impl Runtime {
    pub fn new() -> Self {
        let runtime_id = leptos_reactive::create_runtime();
        let (scope, _) = leptos_reactive::raw_scope_and_disposer(runtime_id);
        Self { runtime_id, scope }
    }

    pub fn create_batch_effect<T, Effect>(&self, effect: Effect)
    where
        T: 'static,
        Effect: Fn(Option<T>) -> T + 'static,
    {
        self.scope
            .batch(|| leptos_reactive::create_effect(self.scope, effect))
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime_id.dispose();
    }
}

impl<Target> CreateState<Target> for Runtime
where
    Target: 'static,
{
    type Reactive = State<Target>;

    fn create_state(&self, value: Target) -> State<Target> {
        let signal = leptos_reactive::create_rw_signal(self.scope, value);
        State(signal)
    }
}

impl<Target, Output> CreateStateSlice<Target, Output> for Runtime
where
    Target: 'static,
    Output: Clone + PartialEq + 'static,
{
    type State = State<Target>;
    type Reactive = StateSlice<Output>;

    fn create_slice(
        &self,
        state: State<Target>,
        getter: impl Fn(&Target) -> Output + Clone + Copy + 'static,
        setter: impl Fn(&mut Target, Output) + Clone + Copy + 'static,
    ) -> StateSlice<Output> {
        let getter = leptos_reactive::create_memo(self.scope, move |_| state.with(getter));
        let setter = move |value| state.update(|x| setter(x, value));
        StateSlice(getter.into(), setter.mapped_signal_setter(self.scope))
    }
}
