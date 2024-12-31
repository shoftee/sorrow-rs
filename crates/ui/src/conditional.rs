use leptos::{logging::log, prelude::*};

#[slot]
pub struct Main {
    #[prop(into)]
    condition: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct ElseIf {
    #[prop(into)]
    condition: Signal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn Conditional(
    main: Main,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> AnyView {
    if main.condition.get() {
        log!("Main condition.");
        (main.children)().into_any()
    } else if let Some(else_if) = else_if.iter().find(|i| i.condition.get()) {
        (else_if.children)().into_any()
    } else if let Some(fallback) = &fallback {
        (fallback.children)().into_any()
    } else {
        ().into_any()
    }
}
