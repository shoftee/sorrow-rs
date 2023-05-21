use std::ops::Mul;

use sorrow_core::{
    communication::{
        Command, Notification, PartialResourceState, PartialTimeState, ReactiveResourceState,
        ReactiveTimeState, ResourceState, TimeControl, TimeState,
    },
    reactive::{IntoReactive, Runtime},
    timers::{DeltaTime, Ticker, TimeSpan},
    utils::channel::{Receiver, Sender},
};

pub struct WorldQueues {
    pub commands: Receiver<Command>,
    pub notifications: Sender<Notification>,
}

pub struct World {
    runtime: Runtime,
    world_queues: WorldQueues,

    delta_time: DeltaTime,
    game_ticks: Ticker,
    time_state: ReactiveTimeState,

    resource_controller: ResourceController,
}

impl World {
    pub fn new(world_queues: WorldQueues) -> Self {
        let runtime = Runtime::new();

        let time_state = TimeState::default().into_reactive(&runtime);

        let resource_controller = ResourceController::new(&runtime);

        Self {
            runtime,
            world_queues,

            delta_time: DeltaTime::new(),
            game_ticks: Ticker::new(std::time::Duration::from_millis(200)),
            time_state,

            resource_controller,
        }
    }

    pub fn activate(&mut self) {
        let sender = &self.world_queues.notifications;
        sender.send(Notification::Initialized);

        // set up updates for time state
        {
            let acceleration = self.time_state.acceleration;
            let paused = self.time_state.paused;

            self.runtime.create_batch_effect({
                let sender = sender.clone();
                move |_| {
                    sender.send(Notification::StateChanged {
                        time: Some(PartialTimeState {
                            acceleration: Some(acceleration.get()),
                            paused: Some(paused.get()),
                        }),
                        resource: None,
                    })
                }
            });
        }

        // set up updates for resources
        {
            let catnip = self.resource_controller.state.catnip;

            self.runtime.create_batch_effect({
                let sender = sender.clone();
                move |_| {
                    sender.send(Notification::StateChanged {
                        time: None,
                        resource: Some(PartialResourceState {
                            catnip: Some(catnip.get()),
                        }),
                    })
                }
            })
        }
    }

    pub fn update(&mut self) {
        let receiver = &self.world_queues.commands;
        while let Some(command) = receiver.try_recv() {
            match command {
                Command::TimeControl(time_control) => match time_control {
                    TimeControl::SetAcceleration(acceleration) => {
                        self.time_state.acceleration.set(acceleration)
                    }
                    TimeControl::Start => self.time_state.paused.set(false),
                    TimeControl::Pause => self.time_state.paused.set(true),
                },
                Command::Initialize => {
                    unreachable!("Update should never be called for the Initialize command.")
                }
            }
        }

        if self.time_state.paused.get() {
            return;
        }

        // advance system time
        self.delta_time.update();
        let system_delta = self.delta_time.delta();

        // apply time acceleration
        let game_delta = system_delta * self.time_state.acceleration.get().into();

        // simulate separate ticks in case the delta is too long
        for segment in game_delta.segments_iter(self.game_ticks.tick_duration()) {
            self.game_ticks.advance(segment);

            self.resource_controller.update(segment);
        }
    }
}

struct ResourceController {
    state: ReactiveResourceState,
}

impl ResourceController {
    const CATNIP_RATE: AmountPerTimeSpan = AmountPerTimeSpan(0.125);

    fn new(runtime: &Runtime) -> Self {
        Self {
            state: ResourceState::default().into_reactive(runtime),
        }
    }

    fn update(&mut self, delta: TimeSpan) {
        self.state
            .catnip
            .update(|v| *v += Self::CATNIP_RATE * delta)
    }
}

struct AmountPerTimeSpan(f64);

impl Mul<TimeSpan> for AmountPerTimeSpan {
    type Output = f64;

    fn mul(self, rhs: TimeSpan) -> Self::Output {
        self.0 * rhs.value()
    }
}
