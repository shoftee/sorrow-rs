mod components;
mod endpoint;
mod events;
mod formatter;
mod layout;
mod store;

pub fn start() {
    tracing_wasm::set_as_global_default_with_config(tracing_wasm::WASMLayerConfig::default());
    mount();
}

leptos_i18n::load_locales!();
use i18n::*;

fn mount() {
    use leptos::prelude::*;
    use leptos_meta::*;

    use self::layout::*;

    mount_to_body(|| {
        provide_meta_context();
        self::events::provide_keyboard_events_context();

        let (endpoint, store) = endpoint::connect();
        endpoint::provide_endpoint(endpoint);
        store::provide_store(store);

        view! {
            <I18nContextProvider>
                <Title text="Observable Sorrow"/>
                <App />
            </I18nContextProvider>
        }
    })
}
