use sorrow_derive::Reactive;

#[derive(Reactive)]
struct Test {
    number: i32,
}

fn main() {
    use sorrow_core::reactive::*;

    let runtime = Runtime::new();
    let reactive = (Test { number: 42 }).into_reactive(&runtime);
    let number = reactive.number.get();
    assert_eq!(number, 42);
}
