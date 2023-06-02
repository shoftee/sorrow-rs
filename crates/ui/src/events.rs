use leptos::*;

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

        let track_keys = move |ev: ev::KeyboardEvent| {
            ctrl.set(ev.ctrl_key());
            shift.set(ev.shift_key());
            alt.set(ev.alt_key());
        };
        window_event_listener(ev::keydown, track_keys);
        window_event_listener(ev::keyup, track_keys);

        window_event_listener(ev::visibilitychange, move |_| {
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

pub fn use_keyboard_events(cx: Scope) -> KeyboardEvents {
    use_context(cx).expect("keyboard events not provided in context")
}
