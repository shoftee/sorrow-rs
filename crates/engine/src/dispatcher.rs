use gloo_timers::callback::Interval;
use gloo_worker::{HandlerId, WorkerScope};

use sorrow_core::{
    communication::{Command, Notification},
    utils::{channel, RcCell, Receiver, Sender},
};

use super::{
    worker::Worker,
    world::{World, WorldQueues},
};

struct WorkerQueues {
    commands: Sender<Command>,
    notifications: Receiver<Notification>,
}

pub struct Dispatcher {
    world: RcCell<World>,
    worker_queues: WorkerQueues,

    interval: Option<Interval>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (worker_queues, world_queues) = queues();
        Self {
            world: World::new(world_queues).into(),
            worker_queues,

            interval: None,
        }
    }

    pub fn accept(&mut self, scope: WorkerScope<Worker>, id: HandlerId, command: Command) {
        match command {
            Command::Initialize => {
                self.world.borrow_mut().activate();

                self.interval.replace(Interval::new(50, {
                    let world = self.world.clone();
                    let notifications = self.worker_queues.notifications.clone();
                    move || {
                        world.borrow_mut().update();

                        while let Some(notification) = notifications.try_recv() {
                            scope.respond(id, notification);
                        }
                    }
                }));
            }
            _ => {
                let sender = &self.worker_queues.commands;
                sender.send(command);
            }
        }
    }
}

fn queues() -> (WorkerQueues, WorldQueues) {
    let (command_sender, command_receiver) = channel();
    let (notification_sender, notification_receiver) = channel();

    let world_queues = WorldQueues {
        commands: command_receiver,
        notifications: notification_sender,
    };
    let worker_queues = WorkerQueues {
        commands: command_sender,
        notifications: notification_receiver,
    };

    (worker_queues, world_queues)
}
