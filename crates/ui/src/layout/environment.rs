use leptos::prelude::*;
use sorrow_core::{communication::*, state::time::RunningState};

use crate::state::{send_command, use_state_signals};

#[component]
pub fn EnvironmentContainer() -> impl IntoView {
    let state = use_state_signals();

    let running_state = Signal::derive(move || state.running_state.get());

    view! {
        <section class="environment-area unscroll-y flex flex-col gap-2">
            <div>"Calendar goes here"</div>
            <div>"You are a kitten in a catnip forest."</div>
            <div class="flex flex-col">
                <div class="btn-group">
                    <button type="button" class="btn btn-outline-secondary">"Clear log"</button>
                    <PawseButton running_state=running_state />
                </div>
            </div>
            <div class="overflow-y-hidden flex-grow flex flex-col text-sm fade-down-to-transparent space-y-4">
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
                <EpochSection />
            </div>
        </section>
    }
}

#[component]
fn EpochSection() -> impl IntoView {
    view! {
        <div class="epoch-section">
            <div class="epoch-title">"Epoch Title"</div>
            <div class="epoch-event">"Epoch Event"</div>
        </div>
    }
}

#[component]
fn PawseButton(running_state: Signal<RunningState>) -> impl IntoView {
    let pawsed = Memo::new(move |_| matches!(running_state.get(), RunningState::Paused));

    let toggle = move |_| {
        send_command(if pawsed.get() {
            Intent::TimeControl(TimeControl::Start)
        } else {
            Intent::TimeControl(TimeControl::Pause)
        });
    };

    view! {
        <button type="button" class="btn btn-outline-secondary" class:active=pawsed on:click=toggle>{
            move || if pawsed.get() { "Unpawse" } else { "Pawse" }
        }</button>
    }
}
