use bevy::{
    app::{Last, Plugin},
    prelude::{DetectChanges, EventWriter, IntoSystemConfigs, NonSendMut, Query, Ref},
};
use sorrow_core::{
    communication::Notification,
    state::{calendar::PartialCalendarState, resources::ResourceState, PartialState},
};

use crate::simulation::{
    calendar::{Day, Season, Year},
    resources::{Amount, Kind},
};

use super::OutputEvent;

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct ChangeBufferPlugin;

impl Plugin for ChangeBufferPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_non_send_resource(PartialState::default())
            .add_systems(Last, detect_changes.in_set(schedule::Main));
    }
}

fn detect_changes(
    mut state: NonSendMut<PartialState>,
    resources: Query<(&Kind, Ref<Amount>)>,
    calendar: Query<(Ref<Day>, Ref<Season>, Ref<Year>)>,
    mut outputs: EventWriter<OutputEvent>,
) {
    {
        let mut has_resource_changes = false;
        let mut resource_state = ResourceState::default();
        for (kind, amount) in resources.iter() {
            if amount.is_changed() {
                let amount_state = resource_state.amounts.get_state_mut(&kind.0);
                *amount_state = Some((*amount).into());
                has_resource_changes = true;
            }
        }

        if has_resource_changes {
            state.resources = Some(resource_state);
        }
    }

    if let Ok(calendar) = calendar.get_single() {
        let mut has_calendar_changes = false;
        let mut calendar_state = PartialCalendarState::default();
        let day = &calendar.0;
        if day.is_changed() {
            calendar_state.day = Some(day.0);
            has_calendar_changes = true;
        }

        let season = &calendar.1;
        if season.is_changed() {
            calendar_state.season = Some(season.0);
            has_calendar_changes = true;
        }

        let year = &calendar.2;
        if year.is_changed() {
            calendar_state.year = Some(year.0);
            has_calendar_changes = true;
        }

        if has_calendar_changes {
            state.calendar = Some(calendar_state);
        }
    }

    if state.is_changed() {
        let changed = std::mem::take(state.as_mut());
        outputs.send(OutputEvent(Notification::StateChanged(changed)));
    }
}
