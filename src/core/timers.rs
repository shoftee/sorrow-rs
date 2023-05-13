use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan(f64);

impl TimeSpan {
    pub const ZERO: TimeSpan = TimeSpan(0f64);

    pub fn ticks_iter(&self, max_span: impl Into<TimeSpan>) -> TicksIterator {
        TicksIterator {
            max_span: max_span.into(),
            remaining: *self,
        }
    }
}

impl From<Duration> for TimeSpan {
    fn from(value: Duration) -> Self {
        Self(value.as_seconds_f64())
    }
}

impl std::ops::Sub for TimeSpan {
    type Output = TimeSpan;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

pub struct TicksIterator {
    max_span: TimeSpan,
    remaining: TimeSpan,
}

impl Iterator for TicksIterator {
    type Item = TimeSpan;

    fn next(&mut self) -> Option<Self::Item> {
        let segment = if self.remaining > self.max_span {
            self.max_span
        } else {
            self.remaining
        };

        self.remaining -= segment;

        if segment.0 > 0.0 {
            Some(segment)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TickPair {
    fractional: f64,
    whole: u64,
}

impl TickPair {
    pub fn fractional(&self) -> f64 {
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
    tick_duration: TimeSpan,
    pub absolute: TickPair,
    pub delta: TickPair,
}

impl Ticker {
    pub fn new(tick_duration: impl Into<TimeSpan>) -> Self {
        Self {
            tick_duration: tick_duration.into(),
            absolute: Default::default(),
            delta: Default::default(),
        }
    }

    pub fn advance(&mut self, delta_span: TimeSpan) {
        let last = self.absolute;

        self.absolute.fractional += delta_span.0 / self.tick_duration.0;
        self.absolute.whole = self.absolute.fractional.floor() as u64;

        self.delta = self.absolute - last;
    }
}

pub struct DeltaTime {
    last: OffsetDateTime,
    absolute: f64,
    delta: f64,
}

impl DeltaTime {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            last: OffsetDateTime::now_utc(),
            absolute: 0f64,
            delta: 0f64,
        }
    }

    pub fn update(&mut self) {
        let current_time = OffsetDateTime::now_utc();
        let last_time = self.last;
        self.last = current_time;

        let delta = (current_time - last_time).as_seconds_f64();
        self.delta = delta;
        self.absolute += delta;
    }

    pub fn absolute(&self) -> TimeSpan {
        TimeSpan(self.absolute)
    }

    pub fn delta(&self) -> TimeSpan {
        TimeSpan(self.delta)
    }
}
