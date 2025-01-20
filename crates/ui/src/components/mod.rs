pub mod conditional;
pub mod numbers;
pub mod tooltip;

use leptos::prelude::*;
use leptos::{component, IntoView};

use sorrow_core::communication::Intent;

use crate::endpoint::use_endpoint;

#[component]
pub fn Button(#[prop(into)] intent: Signal<Intent>, children: ChildrenFn) -> impl IntoView {
    view! {
        <button
            class="btn" type="button" on:click=move |_| use_endpoint().send(intent.get())
        >
            {children()}
        </button>
    }
}
