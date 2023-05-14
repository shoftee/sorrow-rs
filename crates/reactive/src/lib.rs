pub trait Get<T>
where
    T: Clone,
{
    fn get(&self) -> T;
}

pub trait Set<T> {
    fn set(&self, new_value: T);
}

pub trait With<T> {
    fn with<Output>(&self, f: impl FnOnce(&T) -> Output) -> Output;
}

pub trait Update<T> {
    fn update(&self, f: impl FnOnce(&mut T));
}

pub trait StateSlice<T> {
    fn get(&self) -> T;
    fn set(&self, new_value: T);
}

pub trait CreateState<Target>
where
    Target: 'static,
{
    type Reactive: With<Target> + Update<Target>;

    fn create_state(&self, value: Target) -> Self::Reactive;
}

pub trait CreateStateSlice<Target, Output>
where
    Target: 'static,
    Output: Clone + PartialEq + 'static,
{
    type State: With<Target> + Update<Target>;
    type Reactive: Get<Output> + Set<Output>;

    fn create_slice(
        &self,
        state: Self::State,
        getter: impl Fn(&Target) -> Output + Clone + Copy + 'static,
        setter: impl Fn(&mut Target, Output) + Clone + Copy + 'static,
    ) -> Self::Reactive;
}
