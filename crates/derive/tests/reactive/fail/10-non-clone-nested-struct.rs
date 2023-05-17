use sorrow_derive::Reactive;

#[derive(Reactive)]
struct OuterTest {
    #[reactive(nested)]
    nested: InnerTest,
}

#[derive(Reactive)]
struct InnerTest {
    test_field: i32,
}

fn main() {}
