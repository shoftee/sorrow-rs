use super::{GameTick, TimeSpan};

#[derive(Debug, Default, Copy, Clone)]
pub struct TickPair {
    fractional: TimeSpan<GameTick>,
    whole: u64,
}

impl TickPair {
    pub fn fractional(&self) -> TimeSpan<GameTick> {
        self.fractional
    }

    pub fn whole(&self) -> u64 {
        self.whole
    }
}

impl std::ops::Sub for TickPair {
    type Output = TickPair;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            fractional: self.fractional - rhs.fractional,
            whole: self.whole - rhs.whole,
        }
    }
}

#[derive(Debug)]
pub struct Ticker {
    tick_duration: TimeSpan<GameTick>,
    pub absolute: TickPair,
    pub delta: TickPair,
}

impl Ticker {
    pub fn new(tick_duration: impl Into<TimeSpan<GameTick>>) -> Self {
        Self {
            tick_duration: tick_duration.into(),
            absolute: Default::default(),
            delta: Default::default(),
        }
    }

    pub fn span(&self) -> TimeSpan<GameTick> {
        self.tick_duration
    }

    pub fn advance(&mut self, delta: TimeSpan<GameTick>) {
        let last = self.absolute;

        self.absolute.fractional += delta;
        self.absolute.whole = self.absolute.fractional.value().floor() as u64;

        self.delta = self.absolute - last;
    }
}
