use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    prelude::{Commands, Component, IntoSystemConfigs, Query, Res, Resource},
    time::{Fixed, Time, TimePlugin},
};

#[derive(Component, Debug)]
pub struct Ticker {
    scale: f64,

    fractional: f64,
    whole: u64,
    just_ticked: bool,
    delta: f64,
}

#[derive(Resource)]
pub struct TickRate {
    pub seconds_per_tick: f64,
}

impl Default for TickRate {
    fn default() -> Self {
        Self {
            seconds_per_tick: 0.2,
        }
    }
}

impl Ticker {
    pub fn from_scale(scale: u32) -> Self {
        assert!(scale > 0, "scale should never be 0");

        Self {
            scale: scale as f64,

            fractional: 0.0,
            whole: 0,
            just_ticked: false,
            delta: 0.0,
        }
    }

    fn advance(&mut self, delta_ticks: f64) {
        self.delta = round(delta_ticks / self.scale);
        self.fractional = round(self.fractional + self.delta);

        let new_whole = self.fractional.floor() as u64;
        self.just_ticked = self.whole != new_whole;
        self.whole = new_whole;
    }

    pub fn just_ticked(&self) -> bool {
        self.just_ticked
    }
}

fn round(number: f64) -> f64 {
    (number * 1000.0).round() / 1000.0
}

pub mod sets {
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
            .add_systems(FixedUpdate, advance_simulation.in_set(sets::Main));
    }
}

fn spawn(mut cmd: Commands) {
    cmd.spawn(Ticker::from_scale(1));
}

fn advance_simulation(time: Res<Time>, tick_rate: Res<TickRate>, mut tickers: Query<&mut Ticker>) {
    for mut ticker in tickers.iter_mut() {
        ticker.advance(time.delta().as_secs_f64() / tick_rate.seconds_per_tick);
    }
}
