use time::OffsetDateTime;

use super::{Seconds, TimeSpan};

pub struct DeltaTime {
    last: OffsetDateTime,
    absolute: TimeSpan<Seconds>,
    delta: TimeSpan<Seconds>,
}

impl DeltaTime {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            last: OffsetDateTime::now_utc(),
            absolute: Default::default(),
            delta: Default::default(),
        }
    }

    pub fn update(&mut self) {
        let current_time = OffsetDateTime::now_utc();
        let last_time = self.last;
        self.last = current_time;

        self.delta = (current_time - last_time).into();
        self.absolute += self.delta;
    }

    pub fn absolute(&self) -> TimeSpan<Seconds> {
        self.absolute
    }

    pub fn delta(&self) -> TimeSpan<Seconds> {
        self.delta
    }
}
