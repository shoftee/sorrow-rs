use time::OffsetDateTime;

struct Elapsed(f32);

pub struct DeltaTime {
    last: OffsetDateTime,
    elapsed: Elapsed,
}

impl DeltaTime {
    const SECONDS_PER_TICK: f32 = 0.200f32;

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            last: OffsetDateTime::now_utc(),
            elapsed: Elapsed(0f32),
        }
    }

    pub fn update(&mut self) {
        let current_time = OffsetDateTime::now_utc();
        let last_time = self.last;

        self.last = current_time;

        self.elapsed = Elapsed((current_time - last_time).as_seconds_f32());
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.0
    }

    pub fn elapsed_ticks(&self) -> f32 {
        self.elapsed_seconds() / Self::SECONDS_PER_TICK
    }
}
