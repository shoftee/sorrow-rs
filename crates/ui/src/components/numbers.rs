use leptos::{
    either::Either,
    html::{HtmlElement, Span},
    prelude::*,
    tachys::html::class::Class,
};

use crate::{
    formatter::{Formatter, ShowSign},
    store::{use_global_store, GlobalStoreFields, PreferencesStoreFields},
};

pub fn number_span<I: IntoView>(inner: I) -> HtmlElement<Span, (Class<&'static str>,), (I,)> {
    leptos::html::span().class("number").child(inner)
}

#[allow(dead_code)]
#[component]
pub fn IntegerView(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] show_sign: ShowSign,
) -> Either<impl IntoView, impl IntoView> {
    match show_sign {
        ShowSign::NegativeOnly => Either::Left(number_span(move || format!("{}", value.get()))),
        ShowSign::Always => Either::Right(number_span(move || format!("{:+}", value.get()))),
    }
}

#[component]
pub fn DecimalView(
    #[prop(into)] value: Signal<f64>,
    #[prop(optional)] show_sign: ShowSign,
) -> impl IntoView {
    let store = use_global_store().preferences();
    let precision = Memo::new(move |_| store.precision().get());

    number_span(move || Formatter::format(value.get(), show_sign, precision.get()))
}
