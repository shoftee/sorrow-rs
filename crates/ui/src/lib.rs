use tracing_wasm::WASMLayerConfig;

mod events;
mod formatter;
mod layout;
mod state;

mod components;
pub mod endpoint;

pub fn start() {
    tracing_wasm::set_as_global_default_with_config(WASMLayerConfig::default());
    mount();
}

fn mount() {
    use leptos::prelude::*;
    use leptos_meta::*;

    use self::layout::*;

    mount_to_body(|| {
        provide_meta_context();
        self::events::provide_keyboard_events_context();
        self::state::provide_global_store();
        self::endpoint::provide_endpoint_context();

        view! {
           <Title text="Obserable Sorrow"/>
           <App />
        }
    })
}
