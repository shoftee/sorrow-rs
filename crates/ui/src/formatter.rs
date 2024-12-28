// almost shamelessly copied from Sandcastle Builder
// https://github.com/eternaldensity/Sandcastle-Builder/blob/master/redundancy.js

use sorrow_core::state::Precision;

struct Breakpoint {
    limit: f64,
    divisor: f64,
    suffix: &'static str,
}

impl Breakpoint {
    const fn new(limit: f64, divisor: f64, suffix: &'static str) -> Self {
        Self {
            limit,
            divisor,
            suffix,
        }
    }
}

static BREAKPOINTS: [Breakpoint; 15] = [
    Breakpoint::new(1e210, 1e210, "Q"),
    Breakpoint::new(1e_42, 1e_42, "W"),
    Breakpoint::new(1e_39, 1e_39, "L"),
    Breakpoint::new(1e_36, 1e_36, "F"),
    Breakpoint::new(1e_33, 1e_33, "H"),
    Breakpoint::new(1e_30, 1e_30, "S"),
    Breakpoint::new(1e_27, 1e_27, "U"),
    Breakpoint::new(1e_24, 1e_24, "Y"),
    Breakpoint::new(1e_21, 1e_21, "Z"),
    Breakpoint::new(1e_18, 1e_18, "E"),
    Breakpoint::new(1e_15, 1e_15, "P"),
    Breakpoint::new(1e_12, 1e_12, "T"),
    Breakpoint::new(1e__9, 1e__9, "G"),
    Breakpoint::new(1e__6, 1e__6, "M"),
    // Start displaying K only when we're almost at 5 digits.
    Breakpoint::new(9e__3, 1e__3, "K"),
];

static UNIT_CAPACITY: usize = 5;

pub struct Formatter;

impl Formatter {
    pub fn format(number: f64, show_sign: ShowSign, precision: Precision) -> String {
        assert!(!number.is_nan());

        let mut scaled_number = number;
        let mut unit = String::with_capacity(UNIT_CAPACITY);

        // determine suffixes
        for &Breakpoint {
            limit,
            divisor,
            suffix,
        } in BREAKPOINTS.iter()
        {
            let rounded = scaled_number.round_with_precision(precision);

            // At the high-end of f64, round_with_precision() will return Infinity.
            // If this happens, ignore the rounding operation for this scale.
            if rounded.is_finite() {
                scaled_number = rounded;
            }

            // build up list of suffixes
            while scaled_number + f64::EPSILON >= limit {
                unit.push_str(suffix);
                scaled_number /= divisor;
            }
        }

        // one final round before we stringify the rest
        scaled_number = scaled_number.round_with_precision(precision);

        match show_sign {
            ShowSign::NegativeOnly => format!(
                "{number:.precision$}{unit}",
                number = scaled_number,
                precision = precision.into(),
                unit = unit,
            ),
            ShowSign::Always => format!(
                "{number:+.precision$}{unit}",
                number = scaled_number,
                precision = precision.into(),
                unit = unit,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ShowSign {
    #[default]
    NegativeOnly,
    Always,
}

pub trait Roundable {
    fn round_with_precision(self, precision: Precision) -> f64;
    fn trunc_with_precision(self, precision: Precision) -> f64;
}

impl Roundable for f64 {
    fn round_with_precision(self, precision: Precision) -> f64 {
        let precision: usize = precision.into();
        let scale = precision.pow(10) as f64;
        (self * scale * (1f64 + f64::EPSILON)).round() / scale
    }

    fn trunc_with_precision(self, precision: Precision) -> f64 {
        let precision: usize = precision.into();
        let scale = precision.pow(10) as f64;
        (self * scale * (1f64 + f64::EPSILON)).trunc() / scale
    }
}
