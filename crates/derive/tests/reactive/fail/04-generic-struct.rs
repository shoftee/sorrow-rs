use sorrow_derive::Reactive;

#[derive(Reactive)]
pub struct Generic<T> {
    inner: T,
}

fn main() {}
