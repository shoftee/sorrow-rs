use leptos_reactive::{
    create_effect, create_rw_signal, create_slice, RuntimeId, RwSignal, Scope, Signal, SignalGet,
    SignalSetter,
};

pub struct Runtime {
    runtime: RuntimeId,
    scope: Scope,
}

impl Runtime {
    pub fn create_effect<T, Effect>(&self, effect: Effect)
    where
        T: 'static,
        Effect: Fn(Option<T>) -> T + 'static,
    {
        self.scope.batch(|| create_effect(self.scope, effect))
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime.dispose();
    }
}

pub struct State<T>(RwSignal<T>)
where
    T: 'static;

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for State<T> {}

pub struct DerivedState<T>(Signal<T>, SignalSetter<T>)
where
    T: Clone + 'static;

impl<T> Clone for DerivedState<T>
where
    T: Clone + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for DerivedState<T> where T: Clone + 'static {}

impl<S> DerivedState<S>
where
    S: Clone + 'static,
{
    pub fn get(&self) -> S {
        self.0.get()
    }

    pub fn set(&self, new_value: S) {
        self.1.set(new_value);
    }
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = leptos_reactive::create_runtime();
        let (scope, _) = leptos_reactive::raw_scope_and_disposer(runtime);
        Self { runtime, scope }
    }

    pub fn state<S>(&self, value: S) -> State<S>
    where
        S: 'static,
    {
        let signal = create_rw_signal(self.scope, value);
        State(signal)
    }

    pub fn derived_state<S, Output, Getter, Setter>(
        &self,
        target: State<S>,
        getter: Getter,
        setter: Setter,
    ) -> DerivedState<Output>
    where
        S: 'static,
        Output: Clone + PartialEq + 'static,
        Getter: Fn(&S) -> Output + Clone + Copy + 'static,
        Setter: Fn(&mut S, Output) + Clone + Copy + 'static,
    {
        let (output_signal, output_setter) = create_slice(self.scope, target.0, getter, setter);
        DerivedState(output_signal, output_setter)
    }
}

pub trait CreateReactive<Source> {
    type Target;

    fn create_reactive(&self, value: Source) -> Self::Target;
}
