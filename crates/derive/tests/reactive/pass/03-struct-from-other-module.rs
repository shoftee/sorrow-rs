mod inner {
    use sorrow_derive::Reactive;

    #[derive(Reactive)]
    pub struct Inner {
        pub test: i32,
    }
}

mod other {
    use sorrow_derive::Reactive;

    #[derive(Reactive)]
    struct Outer {
        #[reactive(nested)]
        inner: super::inner::Inner,
    }
}

fn main() {}
