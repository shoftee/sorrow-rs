use sorrow_derive::Reactive;

#[derive(Reactive)]
struct OuterTest {
    #[reactive]
    inner: InnerTest,
}

struct InnerTest {
    test_field: i32,
}

fn main() {}
