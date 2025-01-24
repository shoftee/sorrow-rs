use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::*,
};
use sorrow_core::{
    communication::EngineUpdate,
    state::calendar::{PartialCalendarState, SeasonKind},
};

use crate::{io::UpdatedEvent, schedules::BufferChanges, simulation::ticker::Ticker};

#[derive(Component)]
struct DayTicker;

#[derive(Component)]
pub struct Calendar;

#[derive(Component)]
pub struct Year(pub usize);

#[derive(Component)]
pub struct Season(pub SeasonKind);

#[derive(Component)]
pub struct Day(pub i16);

pub mod sets {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct CalendarPlugin;

impl Plugin for CalendarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, advance_calendar.in_set(sets::Main))
            .add_systems(BufferChanges, detect_calendar_changes);
    }
}

fn spawn(mut cmd: Commands) {
    cmd.spawn((Calendar, Year(0), Season(SeasonKind::Spring), Day(0)));
    cmd.spawn((DayTicker, Ticker::from_scale(10)));
}

fn advance_calendar(
    day_ticker: Single<&Ticker, With<DayTicker>>,
    mut calendar: Single<(&mut Day, &mut Season, &mut Year), With<Calendar>>,
) {
    if !day_ticker.just_ticked() {
        return;
    }

    let day = &mut calendar.0;
    let mut is_new_season = false;
    if day.0 == 99 {
        day.0 = 0;
        is_new_season = true;
    } else {
        day.0 += 1;
    }

    let mut is_new_year = false;
    if is_new_season {
        let season = &mut calendar.1;
        if season.0 == SeasonKind::Winter {
            is_new_year = true;
        }

        season.0 = match season.0 {
            SeasonKind::Spring => SeasonKind::Summer,
            SeasonKind::Summer => SeasonKind::Autumn,
            SeasonKind::Autumn => SeasonKind::Winter,
            SeasonKind::Winter => SeasonKind::Spring,
        };
    }

    if is_new_year {
        let year = &mut calendar.2;
        year.0 += 1;
    }
}

fn detect_calendar_changes(
    calendar: Query<(Ref<Day>, Ref<Season>, Ref<Year>)>,
    mut updates: EventWriter<UpdatedEvent>,
) {
    if let Ok(calendar) = calendar.get_single() {
        let mut has_changes = false;
        let mut state = PartialCalendarState::default();
        let day = &calendar.0;
        if day.is_changed() {
            state.day = Some(day.0);
            has_changes = true;
        }

        let season = &calendar.1;
        if season.is_changed() {
            state.season = Some(season.0);
            has_changes = true;
        }

        let year = &calendar.2;
        if year.is_changed() {
            state.year = Some(year.0);
            has_changes = true;
        }

        if has_changes {
            updates.send(EngineUpdate::CalendarChanged(state).into());
        }
    }
}
