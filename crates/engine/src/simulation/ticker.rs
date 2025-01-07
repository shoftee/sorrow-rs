use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::{Commands, Component, IntoSystemConfigs, Query, Res, Resource},
    time::{Fixed, Time, TimePlugin},
};

type Fixed10 = fixed::FixedU64<fixed::types::extra::U10>;

#[derive(Component, Debug)]
pub struct Ticker {
    scale: Fixed10,

    fractional: Fixed10,
    whole: u64,
    just_ticked: bool,
    delta: Fixed10,
}

#[derive(Resource)]
pub struct TickRate {
    pub seconds_per_tick: Fixed10,
}

impl Default for TickRate {
    fn default() -> Self {
        Self {
            seconds_per_tick: Fixed10::from_num(0.2),
        }
    }
}

impl Ticker {
    pub fn from_scale(scale: u64) -> Self {
        assert!(scale > 0, "rate should never be 0");

        Self {
            scale: Fixed10::from_num(scale),

            fractional: Fixed10::ZERO,
            whole: 0,
            just_ticked: false,
            delta: Fixed10::ZERO,
        }
    }

    fn advance(&mut self, delta_ticks: Fixed10) {
        let fixed_delta = delta_ticks / self.scale;
        let new_fractional = self.fractional + fixed_delta;
        self.delta = new_fractional - self.fractional;
        self.fractional = new_fractional;

        let new_whole = self.fractional.floor().to_num();
        self.just_ticked = self.whole != new_whole;
        self.whole = new_whole;
    }

    pub fn just_ticked(&self) -> bool {
        self.just_ticked
    }
}

pub mod schedule {
    use bevy::prelude::SystemSet;

    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Main;
}

pub struct TickerPlugin;

impl Plugin for TickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TimePlugin)
            .insert_resource(Time::<Fixed>::from_hz(5.0))
            .insert_resource(TickRate::default())
            .add_systems(Startup, spawn)
            .add_systems(FixedUpdate, advance_simulation.in_set(schedule::Main));
    }
}

fn spawn(mut cmd: Commands) {
    cmd.spawn(Ticker::from_scale(1));
}

fn advance_simulation(time: Res<Time>, tick_rate: Res<TickRate>, mut tickers: Query<&mut Ticker>) {
    for mut ticker in tickers.iter_mut() {
        ticker.advance(Fixed10::from_num(time.delta().as_secs_f64()) / tick_rate.seconds_per_tick);
    }
}
