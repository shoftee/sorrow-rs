mod interface;

use interface::{header::*, main::*, footer::*};

use leptos::*;
use leptos_meta::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Title text={"Obserable Sorrow"}/>
        <div id="app">
            <div class="app-container">
                <Header/>
                <Main/>
                <Footer/>
            </div>
        </div>
    }
}