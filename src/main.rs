use leptos::{leptos_dom::helpers::IntervalHandle, *};
use leptos_meta::*;
use std::time::Duration;

fn main() {
    mount_to_body(|cx| view! { cx, <App /> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // count_a updates with a fixed interval of 1000 ms, whereas count_b has a dynamic
    // update interval.
    let (count_a, set_count_a) = create_signal(cx, 0_i32);
    let (count_b, set_count_b) = create_signal(cx, 0_i32);

    let (interval, set_interval) = create_signal(cx, 1000);

    use_interval(cx, 1000, move || {
        set_count_a.update(|c| *c = *c + 1);
    });
    use_interval(cx, interval, move || {
        set_count_b.update(|c| *c = *c + 1);
    });

    let (title, _) = create_signal(cx, "Obserable Sorrow".to_string());

    view! { cx,
        <Title text=title />
        <div id="app">
            <div class="app-container">
                <header class="row">
                    <div class="col">
                        <div class="header-start">
                            <div>"Observable Sorrow"</div>
                            <div class="badge bg-success">
                                <i class="bi bi-droplet"></i>
                                " β "
                            </div>
                        </div>
                    </div>
                </header>
                <main class="unscrollable">                
                    <div>"Count A (fixed interval of 1000 ms)"</div>
                    <div>{count_a}</div>
                    <div>"Count B (dynamic interval, currently " {interval} " ms)"</div>
                    <div>{count_b}</div>
                    <input type="number" prop:value={interval} on:input=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse::<u64>() {
                            set_interval(value);
                        }
                    }/>
                </main>
                <footer>
                    <div>
                        "Observable Sorrow is a clone of "
                        <a href="https://kittensgame.com/web/">"Kittens Game"</a>
                        "."
                    </div>
                </footer>
            </div>
        </div>
    }
}

fn use_interval<D, F>(cx: Scope, duration_millis: D, cb: F)
where
    D: Into<MaybeSignal<u64>> + 'static,
    F: Fn() + Clone + 'static,
{
    let duration_millis = duration_millis.into();
    create_effect(cx, move |prev_handle: Option<IntervalHandle>| {
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        }
        let duration = Duration::from_millis(duration_millis.get());
        set_interval_with_handle(cb.clone(), duration).expect("could not create interval")
    });
}
