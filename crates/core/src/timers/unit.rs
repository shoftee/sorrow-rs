use std::{marker::PhantomData, ops::Mul};

use super::TimeSpan;

pub trait TimeUnit: Copy + Default {
    fn seconds_per_unit() -> f64;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Seconds;
impl TimeUnit for Seconds {
    fn seconds_per_unit() -> f64 {
        1_f64
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GameTick;
impl TimeUnit for GameTick {
    fn seconds_per_unit() -> f64 {
        0.2_f64
    }
}

pub struct Rate<U: TimeUnit> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: TimeUnit> Rate<U> {
    pub const fn new(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }
}

impl<U: TimeUnit> Mul<TimeSpan<U>> for Rate<U> {
    type Output = f64;

    fn mul(self, rhs: TimeSpan<U>) -> Self::Output {
        self.value * rhs.value()
    }
}
