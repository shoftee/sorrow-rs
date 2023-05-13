use leptos_reactive::{create_runtime, raw_scope_and_disposer, RuntimeId, Scope};
use time::Duration;

use crate::core::{
    communication::{Command, Notification},
    timers::{DeltaTime, Ticker},
    utils::channel::{Receiver, Sender},
};

pub struct World {
    _runtime: Runtime,
    command_receiver: Receiver<Command>,
    notification_sender: Sender<Notification>,

    state: Option<WorldState>,
}

impl World {
    pub fn new(
        command_receiver: Receiver<Command>,
        notification_sender: Sender<Notification>,
    ) -> Self {
        Self {
            _runtime: Runtime::new(),
            command_receiver,
            notification_sender,

            state: None,
        }
    }

    pub fn activate(&mut self) {
        self.state = Some(WorldState::new());

        self.notification_sender.send(Notification::Initialized);
    }

    pub fn update(&mut self) {
        let state = self.state.as_mut().expect("world state not initialized");
        state.update();

        self.notification_sender
            .send(Notification::LogMessage(format!(
                "Ticks: {:.3}",
                state.ticks.absolute.fractional(),
            )));

        while let Some(cmd) = self.command_receiver.try_recv() {
            self.notification_sender
                .send(Notification::WarnMessage(format!(
                    "Unhandled command: {:?}",
                    cmd,
                )))
        }
    }
}

pub struct Runtime {
    runtime: RuntimeId,
    pub _scope: Scope,
}

impl Runtime {
    fn new() -> Self {
        let runtime = create_runtime();
        let (scope, _) = raw_scope_and_disposer(runtime);
        Self {
            runtime,
            _scope: scope,
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.runtime.dispose();
    }
}

struct WorldState {
    pub delta_time: DeltaTime,
    pub ticks: Ticker,
}

impl WorldState {
    const TICK_DURATION: time::Duration = Duration::milliseconds(200);

    fn new() -> Self {
        Self {
            delta_time: DeltaTime::new(),
            ticks: Ticker::new(Self::TICK_DURATION),
        }
    }

    fn update(&mut self) {
        self.delta_time.update();

        let delta = self.delta_time.delta();
        for segment in delta.ticks_iter(Self::TICK_DURATION) {
            self.ticks.advance(segment);
        }
    }
}
