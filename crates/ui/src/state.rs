use leptos::{
    logging::{log, warn},
    use_context,
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
    pub fn new() -> Self {
        let runtime = Runtime;

        Self {
            options: GameOptionsState::default().into_reactive(&runtime),
            time: TimeState::default().into_reactive(&runtime),
            resource: ResourceState::default().into_reactive(&runtime),
        }
    }
}

#[derive(Clone)]
pub struct CommandSink(std::rc::Rc<Endpoint>);

impl CommandSink {
    pub fn send(&self, command: Command) {
        self.0.send(command);
    }
}

pub struct StateManager {
    signals: StateSignals,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            signals: StateSignals::new(),
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

pub fn provide_state_management_context() {
    let state_manager = StateManager::new();
    leptos::provide_context(state_manager.signals());

    let endpoint = Endpoint::new(move |n| state_manager.accept(n), "./engine.js");
    endpoint.send(Command::Initialize);
    leptos::provide_context(CommandSink(std::rc::Rc::new(endpoint)));
}

pub fn use_state_signals() -> StateSignals {
    leptos::use_context().expect("state signals not provided in context")
}

pub fn use_command_sink() -> CommandSink {
    use_context().expect("command sink not provided in context")
}
