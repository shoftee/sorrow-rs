use sorrow_derive::Reactive;

#[derive(Reactive)]
struct NonCopy {
    non_copy_field: Vec<i32>,
}

fn main() {}
