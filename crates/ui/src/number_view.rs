use leptos::*;

use crate::{
    formatter::{Formatter, ShowSign},
    state::use_state_signals,
};

fn number(inner: impl IntoView) -> HtmlElement<html::Span> {
    leptos_dom::html::span()
        .attr("class", "number")
        .child(inner)
}

#[component]
pub fn IntegerView(show_sign: ShowSign, value: ReadSignal<i64>) -> impl IntoView {
    match show_sign {
        ShowSign::NegativeOnly => number(move || format!("{}", value.get())),
        ShowSign::Always => number(move || format!("{:+}", value.get())),
    }
}

#[component]
pub fn DecimalView(
    #[prop(optional)] show_sign: ShowSign,
    #[prop(into)] value: Signal<f64>,
) -> impl IntoView {
    let signals = use_state_signals();

    number(move || Formatter::format(value.get(), show_sign, signals.options.precision.get()))
}
