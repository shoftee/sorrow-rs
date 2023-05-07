use leptos::*;
use leptos_meta::*;

use super::keyboard_events::*;
use super::layout::*;

use crate::{core::communication::Command, engine::endpoint::EngineEndpoint};

pub fn mount(endpoint: EngineEndpoint) {
    mount_to_body(|cx| {
        provide_meta_context(cx);
        provide_context(cx, KeyboardEvents::new(cx));
        provide_context(cx, endpoint);
        view! { cx, <App/> }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let endpoint = use_endpoint(cx);
    endpoint.send(Command::Increment);
    endpoint.send(Command::Increment);

    view! { cx,
        <Title text="Obserable Sorrow"/>
        <div id="app">
            <div class="app-container">
                <Header/>
                <Center/>
                <Footer/>
            </div>
        </div>
    }
}

fn use_endpoint(cx: Scope) -> EngineEndpoint {
    return use_context(cx).expect("endpoint not provided in context");
}
