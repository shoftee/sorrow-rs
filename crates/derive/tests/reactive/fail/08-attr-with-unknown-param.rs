use sorrow_derive::Reactive;

#[derive(Reactive)]
struct OuterTest {
    #[reactive(unexpected)]
    inner: InnerTest,
}

struct InnerTest {
    test_field: i32,
}

fn main() {}
