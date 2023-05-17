pub struct State<T>(leptos_reactive::RwSignal<T>)
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
        leptos_reactive::SignalGet::get(&self.0)
    }

    pub fn set(&self, new_value: T) {
        leptos_reactive::SignalSet::set(&self.0, new_value);
    }

    pub fn with<Output>(&self, f: impl FnOnce(&T) -> Output) -> Output {
        leptos_reactive::SignalWith::with(&self.0, f)
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        leptos_reactive::SignalUpdate::update(&self.0, f);
    }
}

pub struct DependentState<T>(leptos_reactive::Signal<T>, leptos_reactive::SignalSetter<T>)
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
        leptos_reactive::SignalGet::get(&self.0)
    }

    pub fn set(&self, new_value: T) {
        leptos_reactive::SignalSetter::set(&self.1, new_value);
    }
}

pub struct Runtime {
    runtime_id: leptos_reactive::RuntimeId,
    scope: leptos_reactive::Scope,
}

impl Runtime {
    #[allow(clippy::new_without_default)]
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

    pub fn create_state<Target>(&self, value: Target) -> State<Target>
    where
        Target: 'static,
    {
        let signal = leptos_reactive::create_rw_signal(self.scope, value);
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
        let (signal, signal_setter) =
            leptos_reactive::create_slice(self.scope, state.0, getter, setter);
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
