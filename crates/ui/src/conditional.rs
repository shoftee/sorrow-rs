use leptos::*;

#[slot]
pub struct Main {
    #[prop(into)]
    condition: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct ElseIf {
    #[prop(into)]
    condition: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}

#[component]
pub fn Conditional(
    cx: Scope,
    main: Main,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if main.condition.get() {
            (main.children)(cx).into_view(cx)
        } else if let Some(else_if) = else_if.iter().find(|i| i.condition.get()) {
            (else_if.children)(cx).into_view(cx)
        } else if let Some(fallback) = &fallback {
            (fallback.children)(cx).into_view(cx)
        } else {
            ().into_view(cx)
        }
    }
}
