use bevy::{
    app::{Last, MainScheduleOrder, Plugin},
    ecs::schedule::ScheduleLabel,
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BufferChanges;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Recalculate;

pub struct SchedulesPlugin;

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_schedule(BufferChanges).init_schedule(Recalculate);

        let mut main_schedule = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule.insert_before(Last, BufferChanges);
        main_schedule.insert_before(BufferChanges, Recalculate);
    }
}
