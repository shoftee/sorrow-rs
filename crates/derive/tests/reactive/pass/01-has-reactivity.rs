use sorrow_derive::Reactive;

#[derive(Reactive)]
struct Test {
    number: i32,
}

fn main() {
    let runtime = sorrow_reactive::Runtime::new();
    let reactive = sorrow_reactive::IntoReactive::into_reactive(Test { number: 42 }, &runtime);
    let number = reactive.number.get();
    assert_eq!(number, 42);
}
