use leptos::*;
use leptos_meta::*;

use crate::state::provide_endpoint_context;

use super::events::*;
use super::layout::*;

pub fn mount() {
    mount_to_body(|cx| {
        provide_meta_context(cx);
        provide_keyboard_events_context(cx);
        provide_endpoint_context(cx);
        view! { cx, <App/> }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
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
