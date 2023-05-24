use std::marker::PhantomData;

use super::unit::TimeUnit;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSpan<U: TimeUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: TimeUnit> TimeSpan<U> {
    pub const ZERO: TimeSpan<U> = Self {
        value: 0f64,
        _unit: PhantomData,
    };

    fn from_seconds(value: f64) -> Self {
        Self {
            value: value / U::seconds_per_unit(),
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn convert<V: TimeUnit>(&self) -> TimeSpan<V> {
        TimeSpan::<V>::from_seconds(self.value * U::seconds_per_unit())
    }

    pub fn segments_iter(&self, max_span: impl Into<TimeSpan<U>>) -> SegmentsIterator<U> {
        SegmentsIterator::<U> {
            max_span: max_span.into(),
            remaining: *self,
        }
    }

    pub fn min(lhs: TimeSpan<U>, rhs: TimeSpan<U>) -> TimeSpan<U> {
        if lhs.value < rhs.value {
            lhs
        } else {
            rhs
        }
    }
}

impl<U: TimeUnit> From<time::Duration> for TimeSpan<U> {
    fn from(value: time::Duration) -> Self {
        Self::from_seconds(value.as_seconds_f64())
    }
}

impl<U: TimeUnit> From<std::time::Duration> for TimeSpan<U> {
    fn from(value: std::time::Duration) -> Self {
        Self::from_seconds(value.as_secs_f64())
    }
}

impl<U: TimeUnit> std::ops::Sub for TimeSpan<U> {
    type Output = TimeSpan<U>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
            _unit: PhantomData,
        }
    }
}

impl<U: TimeUnit> std::ops::SubAssign for TimeSpan<U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<U: TimeUnit> std::ops::Add for TimeSpan<U> {
    type Output = TimeSpan<U>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            _unit: PhantomData,
        }
    }
}

impl<U: TimeUnit> std::ops::AddAssign for TimeSpan<U> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<U: TimeUnit> std::ops::Mul<f64> for TimeSpan<U> {
    type Output = TimeSpan<U>;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value * rhs,
            _unit: PhantomData,
        }
    }
}

impl<U: TimeUnit> std::ops::MulAssign<f64> for TimeSpan<U> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

pub struct SegmentsIterator<U: TimeUnit> {
    max_span: TimeSpan<U>,
    remaining: TimeSpan<U>,
}

impl<U: TimeUnit> Iterator for SegmentsIterator<U> {
    type Item = TimeSpan<U>;

    fn next(&mut self) -> Option<Self::Item> {
        let segment = TimeSpan::min(self.max_span, self.remaining);

        self.remaining -= segment;

        if segment.value > 0f64 {
            Some(segment)
        } else {
            None
        }
    }
}
