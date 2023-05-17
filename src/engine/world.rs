use sorrow_derive::Reactive;
use sorrow_reactive::IntoReactive;

use time::Duration;

use crate::core::{
    communication::{Command, Notification, TimeControl},
    time::Acceleration,
    timers::{DeltaTime, Ticker, TimeSpan},
    utils::channel::{Receiver, Sender},
};

pub struct WorldQueues {
    pub commands: Receiver<Command>,
    pub notifications: Sender<Notification>,
}

pub struct World {
    runtime: ::sorrow_reactive::Runtime,
    world_queues: WorldQueues,
    controller: WorldController,
}

impl World {
    pub fn new(world_queues: WorldQueues) -> Self {
        let runtime = ::sorrow_reactive::Runtime::new();
        let controller = WorldController::new(&runtime);
        Self {
            runtime,
            world_queues,
            controller,
        }
    }

    pub fn activate(&mut self) {
        let sender = &self.world_queues.notifications;

        sender.send(Notification::Initialized);
        self.controller.activate(&self.runtime, sender.clone())
    }

    pub fn update(&mut self) {
        let receiver = &self.world_queues.commands;
        while let Some(cmd) = receiver.try_recv() {
            match cmd {
                Command::TimeControl(cmd) => {
                    self.controller.accept(cmd);
                }
                Command::Initialize => unreachable!(),
            }
        }

        self.controller.update();
    }
}

struct WorldController {
    delta_time: DeltaTime,
    ticks: Ticker,
    state: ReactiveWorldState,
}

impl WorldController {
    fn new(runtime: &::sorrow_reactive::Runtime) -> Self {
        Self {
            delta_time: DeltaTime::new(),
            ticks: Ticker::new(Duration::milliseconds(200)),
            state: WorldState::default().into_reactive(runtime),
        }
    }

    fn activate(&self, runtime: &::sorrow_reactive::Runtime, sender: Sender<Notification>) {
        let acceleration = self.state.acceleration;
        let paused = self.state.paused;

        runtime.create_batch_effect(move |_| {
            sender.send(Notification::StateChanged {
                acceleration: acceleration.get(),
                paused: paused.get(),
            })
        });
    }

    fn accept(&mut self, command: TimeControl) {
        match command {
            TimeControl::SetAcceleration(acc) => self.state.acceleration.set(acc),
            TimeControl::Start => self.state.paused.set(false),
            TimeControl::Pause => self.state.paused.set(true),
        }
    }

    fn update(&mut self) {
        self.delta_time.update();

        let delta = self.delta_time.delta();

        if self.state.paused.get() {
            return;
        }

        // apply time acceleration
        let delta = delta * self.state.acceleration.get().into();

        // simulate separate ticks in case the delta is too long
        for segment in delta.segments_iter(self.ticks.tick_duration()) {
            self.update_with_delta(segment);
        }
    }

    fn update_with_delta(&mut self, delta: TimeSpan) {
        self.ticks.advance(delta);
    }
}

#[derive(Debug, Default, Reactive)]
struct WorldState {
    paused: bool,
    acceleration: Acceleration,
}
