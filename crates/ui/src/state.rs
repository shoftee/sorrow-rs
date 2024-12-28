use std::cell::LazyCell;

use leptos::{
    logging::{log, warn},
    prelude::*,
};
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
    fn new() -> Self {
        let runtime = Runtime;

        Self {
            options: GameOptionsState::default().into_reactive(&runtime),
            time: TimeState::default().into_reactive(&runtime),
            resource: ResourceState::default().into_reactive(&runtime),
        }
    }
}

struct StateManager {
    signals: StateSignals,
}

impl StateManager {
    fn signals(&self) -> StateSignals {
        self.signals.clone()
    }

    fn accept(&self, notification: Notification) {
        use Notification::*;

        match notification {
            Initialized => log!("World initialized."),
            LogMessage(msg) => log!("{}", msg),
            WarnMessage(msg) => warn!("{}", msg),
            StateChanged(state) => {
                if let Some(acceleration) = state.acceleration {
                    self.signals.time.acceleration.set(acceleration);
                }
                if let Some(running_state) = state.running_state {
                    self.signals.time.running_state.set(running_state);
                }
                if let Some(resource) = state.resource {
                    if let Some(catnip) = resource.catnip {
                        self.signals.resource.catnip.set(catnip);
                    }
                }
            }
        }
    }
}

const STATE_MANAGER: LazyCell<StateManager> = LazyCell::new(|| StateManager {
    signals: StateSignals::new(),
});
const ENDPOINT: LazyCell<Endpoint> = LazyCell::new(|| {
    Endpoint::new(
        move |notification| STATE_MANAGER.accept(notification),
        "./engine.js",
    )
});

pub fn provide_state_signals_context() {
    provide_context(STATE_MANAGER.signals());
}

pub fn send_command(command: Command) {
    ENDPOINT.send(command);
}

pub fn use_state_signals() -> StateSignals {
    use_context().expect("state signals not provided in context")
}
