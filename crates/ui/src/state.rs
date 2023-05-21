use sorrow_core::{
    communication::{
        Command, Notification, ReactiveResourceState, ReactiveTimeState, ResourceState, TimeState,
    },
    reactive::{IntoReactive, Runtime},
};
use sorrow_engine::endpoint::Endpoint;

#[derive(Clone)]
pub struct StateSignals {
    pub time: ReactiveTimeState,
    pub resource: ReactiveResourceState,
}

impl StateSignals {
    pub fn new(cx: leptos::Scope) -> Self {
        let runtime = Runtime::from_scope(cx);
        let time = TimeState::default().into_reactive(&runtime);
        let resource = ResourceState::default().into_reactive(&runtime);

        Self { time, resource }
    }
}

pub struct StateManager {
    signals: StateSignals,
}

impl StateManager {
    pub fn new(cx: leptos::Scope) -> Self {
        Self {
            signals: StateSignals::new(cx),
        }
    }

    pub fn signals(&self) -> StateSignals {
        self.signals.clone()
    }

    pub fn accept(&self, notification: Notification) {
        use Notification::*;

        match notification {
            Initialized => leptos::log!("World initialized."),
            LogMessage(msg) => leptos::log!("{}", msg),
            WarnMessage(msg) => leptos::warn!("{}", msg),
            _ => leptos::log!("{:?}", notification),
        }
    }
}

pub fn provide_endpoint_context(cx: leptos::Scope) {
    let state_manager = StateManager::new(cx);
    leptos::provide_context(cx, state_manager.signals());

    let endpoint = Endpoint::new(move |n| state_manager.accept(n), "./engine.js");

    endpoint.send(Command::Initialize);

    leptos::provide_context(cx, endpoint);
}

pub fn use_state_signals(cx: leptos::Scope) -> StateSignals {
    leptos::use_context(cx).expect("state signals not provided in context")
}
