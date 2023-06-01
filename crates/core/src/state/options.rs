use sorrow_derive::Reactive;

use super::Precision;

#[derive(Debug, Default, Reactive)]
pub struct GameOptionsState {
    pub precision: Precision,
}
