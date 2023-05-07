use leptos::ev::KeyboardEvent;
use leptos::*;

use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct KeyboardEvents {
    pub ctrl: Memo<bool>,
    pub shift: Memo<bool>,
    pub alt: Memo<bool>,
}

impl KeyboardEvents {
    fn new(cx: Scope) -> Self {
        let ctrl = create_rw_signal(cx, false);
        let shift = create_rw_signal(cx, false);
        let alt = create_rw_signal(cx, false);

        window_keyboard_event_listener("keydown", move |ev| {
            ctrl.set(ev.ctrl_key());
            shift.set(ev.shift_key());
            alt.set(ev.alt_key());
        });
        window_keyboard_event_listener("keyup", move |ev| {
            ctrl.set(ev.ctrl_key());
            shift.set(ev.shift_key());
            alt.set(ev.alt_key());
        });
        document_visibility_change_event_listener(move || {
            ctrl.set(false);
            shift.set(false);
            alt.set(false);
        });

        Self {
            ctrl: create_memo(cx, move |_| ctrl.get()),
            shift: create_memo(cx, move |_| shift.get()),
            alt: create_memo(cx, move |_| alt.get()),
        }
    }
}

pub fn provide_keyboard_events_context(cx: Scope) {
    provide_context(cx, KeyboardEvents::new(cx));
}

fn window_keyboard_event_listener(event_name: &str, cb: impl Fn(KeyboardEvent) + 'static) {
    let handler = Box::new(cb) as Box<dyn FnMut(KeyboardEvent)>;
    let cb = Closure::wrap(handler).into_js_value();
    _ = window().add_event_listener_with_callback(event_name, cb.unchecked_ref());
}

fn document_visibility_change_event_listener(cb: impl Fn() + 'static) {
    let handler = Box::new(cb) as Box<dyn FnMut()>;
    let cb = Closure::wrap(handler).into_js_value();
    _ = document().add_event_listener_with_callback("visibilitychange", cb.unchecked_ref());
}

pub fn use_keyboard_events(cx: Scope) -> KeyboardEvents {
    return use_context(cx).expect("keyboard events not provided in context");
}
