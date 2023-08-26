use sorrow_core::{
    communication::{Command, Notification},
    reactive::{IntoReactive, Runtime},
    state::*,
};
use sorrow_engine::Endpoint;

#[derive(Clone)]
pub struct StateSignals {
    pub options: ReactiveGameOptionsState,
    pub time: ReactiveTimeState,
    pub resource: ReactiveResourceState,
}

impl StateSignals {
    pub fn new(cx: leptos::Scope) -> Self {
        let runtime = Runtime::from_scope(cx);

        Self {
            options: GameOptionsState::default().into_reactive(&runtime),
            time: TimeState::default().into_reactive(&runtime),
            resource: ResourceState::default().into_reactive(&runtime),
        }
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
            StateChanged { time, resource } => {
                if let Some(time) = time {
                    if let Some(acceleration) = time.acceleration {
                        self.signals.time.acceleration.set(acceleration);
                    }
                    if let Some(running_state) = time.running_state {
                        self.signals.time.running_state.set(running_state);
                    }
                }
                if let Some(resource) = resource {
                    if let Some(catnip) = resource.catnip {
                        self.signals.resource.catnip.set(catnip);
                    }
                }
            }
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
