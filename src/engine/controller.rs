use gloo_timers::callback::Interval;
use gloo_worker::{HandlerId, WorkerScope};

use crate::core::{
    communication::{Command, Notification},
    utils::{
        cell::RcCell,
        channel::{channel, Receiver, Sender},
    },
};

use super::{worker::Worker, world::World};

pub struct Controller {
    world: RcCell<World>,

    command_sender: Sender<Command>,
    notification_receiver: Receiver<Notification>,

    interval: Option<Interval>,
}

impl Controller {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = channel();
        let (notification_sender, notification_receiver) = channel();

        Self {
            world: World::new(command_receiver, notification_sender).into(),

            command_sender,
            notification_receiver,

            interval: None,
        }
    }

    pub fn accept(&mut self, scope: WorkerScope<Worker>, id: HandlerId, command: Command) {
        if command == Command::Initialize {
            self.world.borrow_mut().activate();

            let world = self.world.clone();
            let notification_receiver = self.notification_receiver.clone();

            self.interval.replace(Interval::new(50, move || {
                world.borrow_mut().update();

                while let Some(notification) = notification_receiver.try_recv() {
                    scope.respond(id, notification);
                }
            }));
        } else {
            self.command_sender.send(command);
        }
    }
}
