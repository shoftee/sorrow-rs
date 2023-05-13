use leptos_reactive::{RuntimeId, RwSignal, Scope};

pub struct Runtime {
    runtime: RuntimeId,
    scope: Scope,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime.dispose();
    }
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = leptos_reactive::create_runtime();
        let (scope, _) = leptos_reactive::raw_scope_and_disposer(runtime);
        Self { runtime, scope }
    }

    pub fn create_rw_signal<T>(&self, value: T) -> RwSignal<T> {
        leptos_reactive::create_rw_signal(self.scope, value)
    }
}
