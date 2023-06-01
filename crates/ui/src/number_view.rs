use leptos::*;

use crate::{
    formatter::{Formatter, ShowSign},
    state::use_state_signals,
};

fn number(cx: Scope, inner: impl IntoView) -> HtmlElement<html::Span> {
    leptos_dom::html::span(cx)
        .attr("class", "number")
        .child(inner)
}

#[component]
pub fn IntegerView(cx: Scope, show_sign: ShowSign, value: ReadSignal<i64>) -> impl IntoView {
    match show_sign {
        ShowSign::NegativeOnly => number(cx, move || format!("{}", value.get())),
        ShowSign::Always => number(cx, move || format!("{:+}", value.get())),
    }
}

#[component]
pub fn DecimalView(
    cx: Scope,
    #[prop(optional)] show_sign: ShowSign,
    #[prop(into)] value: Signal<f64>,
) -> impl IntoView {
    let signals = use_state_signals(cx);

    number(cx, move || {
        Formatter::format(value.get(), show_sign, signals.options.precision.get())
    })
}
