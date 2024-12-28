use leptos::prelude::*;

#[component]
pub fn ControlsContainer() -> impl IntoView {
    view! {
        <div class="container controls-container">
            <div class="row">
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button">"Gather catnip"</button>
                </div>
                <div class="col">
                    <button class="btn btn-outline-secondary w-100" type="button">"Refine catnip"</button>
                </div>
            </div>
        </div>
    }
}
