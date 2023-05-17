use sorrow_derive::Reactive;

#[derive(Reactive)]
struct Test {
    #[reactive(nested)]
    nested: Nested,
}

#[derive(Reactive, Clone)]
struct Nested {
    number: i32,
}

fn main() {
    let runtime = sorrow_reactive::Runtime::new();
    let reactive = sorrow_reactive::IntoReactive::into_reactive(
        Test {
            nested: Nested { number: 42 },
        },
        &runtime,
    );
    let number = reactive.nested.number.get();
    assert_eq!(number, 42);
}
