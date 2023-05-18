use leptos_reactive::*;

pub struct State<T>(RwSignal<T>)
where
    T: 'static;

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for State<T> {}

impl<T> State<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        SignalGet::get(&self.0)
    }

    pub fn set(&self, new_value: T) {
        SignalSet::set(&self.0, new_value);
    }

    pub fn with<Output>(&self, f: impl FnOnce(&T) -> Output) -> Output {
        SignalWith::with(&self.0, f)
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        SignalUpdate::update(&self.0, f);
    }
}

pub struct DependentState<T>(Signal<T>, SignalSetter<T>)
where
    T: 'static;

impl<T> Clone for DependentState<T>
where
    T: 'static,
{
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for DependentState<T> where T: 'static {}

impl<T> DependentState<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        SignalGet::get(&self.0)
    }

    pub fn set(&self, new_value: T) {
        SignalSetter::set(&self.1, new_value);
    }
}

pub struct Runtime {
    runtime_id: RuntimeId,
    scope: Scope,
}

impl Runtime {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let runtime_id = create_runtime();
        let (scope, _) = raw_scope_and_disposer(runtime_id);
        Self { runtime_id, scope }
    }

    pub fn create_batch_effect<Target, Effect>(&self, effect: Effect)
    where
        Target: 'static,
        Effect: Fn(Option<Target>) -> Target + 'static,
    {
        self.scope.batch(|| create_effect(self.scope, effect))
    }

    pub fn create_state<Target>(&self, value: Target) -> State<Target>
    where
        Target: 'static,
    {
        let signal = create_rw_signal(self.scope, value);
        State(signal)
    }

    pub fn create_dependent<Target, Output>(
        &self,
        state: State<Target>,
        getter: impl Fn(&Target) -> Output + Clone + Copy + 'static,
        setter: impl Fn(&mut Target, Output) + Clone + Copy + 'static,
    ) -> DependentState<Output>
    where
        Output: PartialEq,
    {
        let (signal, signal_setter) = create_slice(self.scope, state.0, getter, setter);
        DependentState(signal, signal_setter)
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime_id.dispose();
    }
}

pub trait IntoReactive {
    type Target;

    fn into_reactive(self, runtime: &Runtime) -> Self::Target;
}
