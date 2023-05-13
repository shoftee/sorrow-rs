use leptos_reactive::{RwSignal, SignalGet};
use time::Duration;

use crate::core::{
    communication::{Command, Notification},
    timers::{DeltaTime, Ticker, TimeSpan},
    utils::channel::{Receiver, Sender},
};

use super::runtime::Runtime;

pub struct World {
    runtime: Runtime,
    command_receiver: Receiver<Command>,
    notification_sender: Sender<Notification>,

    controller: Option<WorldController>,
}

impl World {
    pub fn new(
        command_receiver: Receiver<Command>,
        notification_sender: Sender<Notification>,
    ) -> Self {
        Self {
            runtime: Runtime::new(),
            command_receiver,
            notification_sender,

            controller: None,
        }
    }

    pub fn activate(&mut self) {
        self.controller = Some(WorldController::new(&self.runtime));

        self.notification_sender.send(Notification::Initialized);
    }

    pub fn update(&mut self) {
        let controller = self
            .controller
            .as_mut()
            .expect("world state not initialized");
        controller.update();

        self.notification_sender
            .send(Notification::LogMessage(format!(
                "Ticks: {:.3}",
                controller.state.ticks.absolute.fractional(),
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

struct WorldController {
    delta_time: DeltaTime,
    state: WorldState,
}

impl WorldController {
    fn new(runtime: &Runtime) -> Self {
        Self {
            delta_time: DeltaTime::new(),
            state: WorldState::new(runtime),
        }
    }

    fn update(&mut self) {
        self.delta_time.update();

        let delta = self.delta_time.delta();

        self.state.update(delta);
    }
}

struct WorldState {
    pub ticks: Ticker,
    pub acceleration_factor: RwSignal<f64>,
}

impl WorldState {
    const TICK_DURATION: time::Duration = Duration::milliseconds(200);

    fn new(runtime: &Runtime) -> Self {
        Self {
            ticks: Ticker::new(Self::TICK_DURATION),
            acceleration_factor: runtime.create_rw_signal(1f64),
        }
    }

    fn update(&mut self, delta: TimeSpan) {
        // apply time acceleration
        let delta = delta * self.acceleration_factor.get();

        // simulate separate ticks in case the delta is too long
        for segment in delta.ticks_iter(Self::TICK_DURATION) {
            self.ticks.advance(segment);
        }
    }
}
