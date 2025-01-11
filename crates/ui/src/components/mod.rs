pub mod conditional;
pub mod numbers;

use leptos::prelude::*;
use leptos::{component, IntoView};
use sorrow_core::communication::Intent;

use crate::endpoint::use_endpoint;

#[component]
pub fn Button(#[prop(into)] intent: Signal<Intent>, children: ChildrenFn) -> impl IntoView {
    let endpoint = use_endpoint();
    view! {
        <button class="btn" type="button" on:click=move |_| endpoint.send(intent.get())>
            {children()}
        </button>
    }
}
