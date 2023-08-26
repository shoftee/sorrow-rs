use sorrow_core::{
    communication::{Command, Notification, TimeControl},
    reactive::{IntoReactive, Runtime},
    state::*,
    timers::{DeltaTime, GameTick, Rate, Ticker, TimeSpan},
    utils::{Receiver, Sender},
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

        // set up updates for acceleration
        {
            let acceleration = self.time_state.acceleration;

            let sender = sender.clone();
            self.runtime.create_effect(move |_| {
                sender.send(Notification::StateChanged {
                    time: Some(PartialTimeState {
                        acceleration: Some(acceleration.get()),
                        running_state: None,
                    }),
                    resource: None,
                })
            });
        }

        // set up updates for running state
        {
            let running_state = self.time_state.running_state;

            let sender = sender.clone();
            self.runtime.create_effect(move |_| {
                sender.send(Notification::StateChanged {
                    time: Some(PartialTimeState {
                        acceleration: None,
                        running_state: Some(running_state.get()),
                    }),
                    resource: None,
                })
            });
        }

        // set up updates for resources
        {
            let catnip = self.resource_controller.state.catnip;

            let sender = sender.clone();
            self.runtime.create_effect(move |_| {
                sender.send(Notification::StateChanged {
                    time: None,
                    resource: Some(PartialResourceState {
                        catnip: Some(catnip.get()),
                    }),
                })
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
                    TimeControl::Start => self.time_state.running_state.set(RunningState::Running),
                    TimeControl::Pause => self.time_state.running_state.set(RunningState::Paused),
                },
                Command::Initialize => {
                    unreachable!("Update should never be called for the Initialize command.")
                }
            }
        }

        if matches!(
            self.time_state.running_state.get_untracked(),
            RunningState::Paused
        ) {
            return;
        }

        // advance system time
        self.delta_time.update();
        let delta = self.delta_time.delta();

        // apply time acceleration
        let delta = delta * self.time_state.acceleration.get_untracked().into();

        // convert to game ticks
        let delta = delta.convert::<GameTick>();

        // simulate ticks one by one
        for segment in delta.segments_iter(self.game_ticks.span()) {
            self.game_ticks.advance(segment);

            self.resource_controller
                .update(self.game_ticks.delta.fractional());
        }
    }
}

struct ResourceController {
    state: ReactiveResourceState,
}

impl ResourceController {
    const CATNIP_RATE: Rate<GameTick> = Rate::new(0.125);

    fn new(runtime: &Runtime) -> Self {
        Self {
            state: ResourceState::default().into_reactive(runtime),
        }
    }

    fn update(&mut self, delta: TimeSpan<GameTick>) {
        self.state
            .catnip
            .update(|v| *v += Self::CATNIP_RATE * delta)
    }
}
