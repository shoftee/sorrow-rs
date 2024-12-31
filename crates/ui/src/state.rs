use std::{cell::LazyCell, sync::LazyLock};

use leptos::{
    logging::{log, warn},
    prelude::*,
};
use sorrow_core::{
    communication::{Command, Notification},
    state::*,
};
use sorrow_engine::Endpoint;

#[derive(Clone)]
pub struct OptionSignals {
    pub precision: RwSignal<Precision>,
}

#[derive(Clone)]
pub struct ResourceSignals {
    pub catnip: RwSignal<f64>,
}

#[derive(Clone)]
pub struct StateSignals {
    pub options: OptionSignals,
    pub running_state: RwSignal<RunningState>,
    pub resources: ResourceSignals,
}

impl StateSignals {
    fn new() -> Self {
        Self {
            options: OptionSignals {
                precision: RwSignal::new(Precision::default()),
            },
            running_state: RwSignal::new(RunningState::default()),
            resources: ResourceSignals {
                catnip: RwSignal::new(0.0),
            },
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
                if let Some(running_state) = state.running_state {
                    self.signals.running_state.set(running_state);
                }
                if let Some(resource) = state.resource {
                    if let Some(catnip) = resource.catnip {
                        self.signals.resources.catnip.set(catnip);
                    }
                }
            }
        }
    }
}

static STATE_MANAGER: LazyLock<StateManager> = LazyLock::new(|| StateManager {
    signals: StateSignals::new(),
});

static mut ENDPOINT: LazyCell<Endpoint> = LazyCell::new(|| {
    Endpoint::new(
        move |notification| STATE_MANAGER.accept(notification),
        "./engine.js",
    )
});

pub fn provide_state_signals_context() {
    provide_context(STATE_MANAGER.signals());
    send_command(Command::Load);
}

pub fn send_command(command: Command) {
    #[allow(static_mut_refs)]
    // SAFETY: This is the UI part of a WASM app, we only have one thread.
    unsafe {
        ENDPOINT.send(command);
    }
}

pub fn use_state_signals() -> StateSignals {
    use_context().expect("state signals not provided in context")
}
