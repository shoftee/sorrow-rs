mod interface;

use interface::{footer::*, header::*, main::*};

use leptos::*;
use leptos_meta::*;

use crate::interface::keyboard_events::KeyboardEvents;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, KeyboardEvents::new(cx));
    provide_meta_context(cx);

    view! { cx,
        <Title text="Obserable Sorrow"/>
        <div id="app">
            <div class="app-container">
                <Header/>
                <Main/>
                <Footer/>
            </div>
        </div>
    }
}
