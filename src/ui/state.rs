use leptos::{log, warn};
use leptos_reactive::{
    create_rw_signal, create_slice, provide_context, use_context, Scope, Signal, SignalSetter,
};

use crate::{
    core::communication::{Command, Notification},
    engine::endpoint::Endpoint,
};

#[derive(Default)]
struct State {
    id: u64,
}

#[derive(Clone)]
pub struct StateSignals {
    id: (Signal<u64>, SignalSetter<u64>),
}

impl StateSignals {
    pub fn new(cx: Scope) -> Self {
        let root = create_rw_signal(cx, State::default());
        Self {
            id: create_slice(cx, root, |root| root.id, |root, value| root.id = value),
        }
    }

    pub fn id(&self) -> Signal<u64> {
        self.id.0
    }
}

pub struct StateManager {
    signals: StateSignals,
}

impl StateManager {
    pub fn new(cx: Scope) -> Self {
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
            Initialized => log!("World initialized."),
            LogMessage(msg) => log!("{}", msg),
            WarnMessage(msg) => warn!("{}", msg),
            Started => log!("Started!"),
            Paused => log!("Paused!"),
            StateChanged { id } => self.signals.id.1.set(id),
        }
    }
}

pub fn provide_endpoint_context(cx: Scope) {
    let state_manager = StateManager::new(cx);
    provide_context(cx, state_manager.signals());

    let endpoint = Endpoint::new(
        move |notification| state_manager.accept(notification),
        "./engine.js",
    );
    endpoint.send(Command::Initialize);
    provide_context(cx, endpoint);
}

pub fn use_state_signals(cx: Scope) -> StateSignals {
    use_context(cx).expect("state signals not provided in context")
}
