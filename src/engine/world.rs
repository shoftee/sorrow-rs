use leptos_reactive::{create_runtime, raw_scope_and_disposer, RuntimeId, Scope};

use crate::core::{
    communication::{Command, Notification},
    utils::{
        channel::{Receiver, Sender},
        delta_time::DeltaTime,
    },
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
                state.ticks
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
    pub ticks: f32,
}

impl WorldState {
    fn new() -> Self {
        Self {
            delta_time: DeltaTime::new(),
            ticks: 0f32,
        }
    }

    fn update(&mut self) {
        self.delta_time.update();

        self.ticks += self.delta_time.elapsed_ticks()
    }
}
