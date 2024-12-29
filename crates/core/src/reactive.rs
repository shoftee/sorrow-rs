use leptos_reactive::*;

pub struct State<T>(RwSignal<T>)
where
    T: 'static;

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
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

    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        SignalGetUntracked::get_untracked(&self.0)
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
        *self
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

#[derive(Clone)]
pub struct Runtime;

impl Runtime {
    pub fn create_effect<Target, Effect>(&self, effect: Effect)
    where
        Target: 'static,
        Effect: Fn(Option<Target>) -> Target + 'static,
    {
        // NOTE: Use create_render_effect to avoid queue_microtask.
        //       Microtasks don't work inside the worker
        //       because they require a window object.
        leptos_reactive::create_render_effect(effect);
    }

    pub fn create_state<Target>(&self, value: Target) -> State<Target>
    where
        Target: 'static,
    {
        let signal = leptos_reactive::create_rw_signal(value);
        State(signal)
    }

    pub fn create_dependent<Target, Output>(
        &self,
        state: State<Target>,
        getter: impl Fn(&Target) -> Output + Copy + 'static,
        setter: impl Fn(&mut Target, Output) + Copy + 'static,
    ) -> DependentState<Output>
    where
        Output: PartialEq,
    {
        let (signal, signal_setter) = leptos_reactive::create_slice(state.0, getter, setter);
        DependentState(signal, signal_setter)
    }
}

pub trait IntoReactive {
    type Target;

    fn into_reactive(self, runtime: &Runtime) -> Self::Target;
}
