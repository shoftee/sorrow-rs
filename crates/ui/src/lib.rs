mod events;
mod formatter;
mod layout;
mod number_view;
mod state;

mod conditional;

pub fn start() {
    mount();
}

fn mount() {
    use leptos::prelude::*;
    use leptos_meta::*;

    use self::layout::*;

    mount_to_body(|| {
        provide_meta_context();
        self::events::provide_keyboard_events_context();
        self::state::provide_state_signals_context();

        view! {
           <Title text="Obserable Sorrow"/>
           <App />
        }
    })
}
