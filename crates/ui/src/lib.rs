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

fn mount() {
    use i18n::*;
    use leptos::prelude::*;
    use leptos_meta::*;
    use store::GlobalStoreFields;

    use components::conditional::*;

    use self::layout::*;

    fn game_title() -> Memo<&'static str> {
        let i18n = i18n::use_i18n();
        Memo::new(move |_| t_string!(i18n, game.title))
    }

    mount_to_body(|| {
        provide_meta_context();
        self::events::provide_keyboard_events_context();

        let (endpoint, store) = endpoint::connect();
        endpoint::provide_endpoint(endpoint);
        store::provide_store(store);

        let is_loaded = Memo::new(move |_| store.is_loaded().get());

        view! {
            <I18nContextProvider>
                <Conditional>
                    <Main slot condition=is_loaded>
                        <Title text=move || game_title().get() />
                        <App />
                    </Main>
                    <Fallback slot>
                        <div class="w-100 h-100 flex flex-col items-center justify-center">
                            <div class="text-5xl">"Loading..."</div>
                        </div>
                    </Fallback>
                </Conditional>
            </I18nContextProvider>
        }
    })
}
