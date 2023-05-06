use std::time::Duration;

use leptos::{*, leptos_dom::helpers::IntervalHandle};

#[component]
pub(crate) fn Main(cx: Scope) -> impl IntoView {
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

    view! { cx,
        <main class="unscrollable">
            <div class="nav-container">
								<div>"navigation goes here"</div>
                <div class="main-container unscrollable">
										<div class="col unscrollable">"Resources"</div>
										<div class="col unscrollable">
												<div>"Count A (fixed interval of 1000 ms)"</div>
												<div>{count_a}</div>
												<div>"Count B (dynamic interval, currently " {interval} " ms)"</div>
												<div>{count_b}</div>
												<input
														type="number"
														prop:value=interval
														on:input=move |ev| {
																if let Ok(value) = event_target_value(&ev).parse::<u64>() {
																		set_interval(value);
																}
														}
												/>
										</div>
										<div class="col env-container unscrollable">"Calendar and History"</div>
								</div>
            </div>
        </main>
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
