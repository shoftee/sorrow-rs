#[derive(Debug, Clone, Copy, Default)]
pub enum Precision {
    TwoDecimals,
    #[default]
    ThreeDecimals,
}

impl From<Precision> for usize {
    fn from(value: Precision) -> Self {
        match value {
            Precision::TwoDecimals => 2,
            Precision::ThreeDecimals => 3,
        }
    }
}
