use sorrow_derive::Reactive;

#[derive(Reactive)]
struct Test {
    #[reactive(nested)]
    nested: Nested,
}

#[derive(Reactive)]
struct Nested {
    number: i32,
}

fn main() {
    use sorrow_core::reactive::*;

    let runtime = Runtime::new();
    let reactive = (Test {
        nested: Nested { number: 42 },
    })
    .into_reactive(&runtime);

    let number = reactive.nested.number.get();
    assert_eq!(number, 42);
}
