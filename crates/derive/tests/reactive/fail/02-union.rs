use sorrow_derive::Reactive;

#[derive(Reactive)]
#[repr(C)]
pub union TestUnion {
    field_one: i32,
    field_two: i32,
}

fn main() {}
