use leptos::{
    either::Either,
    html::{HtmlElement, Span},
    prelude::*,
    tachys::html::class::Class,
};

use crate::{
    formatter::{Formatter, ShowSign},
    state::use_state_signals,
};

fn number<I: IntoView>(inner: I) -> HtmlElement<Span, (Class<&'static str>,), (I,)> {
    leptos::html::span().class("number").child(inner)
}

#[component]
pub fn IntegerView(
    #[prop(optional)] show_sign: ShowSign,
    #[prop(into)] value: Signal<f64>,
) -> Either<impl IntoView, impl IntoView> {
    match show_sign {
        ShowSign::NegativeOnly => Either::Left(number(move || format!("{}", value.get()))),
        ShowSign::Always => Either::Right(number(move || format!("{:+}", value.get()))),
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
