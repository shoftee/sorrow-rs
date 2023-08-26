mod events;
mod formatter;
mod layout;
mod number_view;
mod state;

mod conditional;

pub fn mount() {
    use leptos::*;
    use leptos_meta::*;

    use self::layout::*;

    mount_to_body(|cx| {
        provide_meta_context(cx);
        self::events::provide_keyboard_events_context(cx);
        self::state::provide_endpoint_context(cx);

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
    })
}
