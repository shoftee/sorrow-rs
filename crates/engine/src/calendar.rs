use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::{Commands, Component, IntoSystemConfigs, Single, With},
};
use sorrow_core::state::calendar::SeasonKind;

use crate::simulation::Ticker;

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

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct CalendarPlugin;

impl Plugin for CalendarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(FixedUpdate, advance_calendar.in_set(schedule::Main));
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
