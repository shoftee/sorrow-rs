use leptos::prelude::*;

#[slot]
pub struct Main {
    #[prop(into)]
    condition: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn Conditional(main: Main, #[prop(optional)] fallback: Option<Fallback>) -> impl IntoView {
    move || {
        if main.condition.get() {
            (main.children)().into_any()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_any()
        } else {
            ().into_any()
        }
    }
}
