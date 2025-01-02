use leptos::prelude::*;
use sorrow_core::{communication::*, state::RunningState};

use crate::state::{send_command, use_state_signals};

#[component]
pub fn EnvironmentContainer() -> impl IntoView {
    let state = use_state_signals();

    let running_state = Signal::derive(move || state.running_state.get());

    view! {
        <div class="calendar-container">"Calendar goes here"</div>
        <section class="history-container unscrollable">
            <div class="game-controls-container">
                <div>"You are a kitten in a catnip forest."</div>
                <div class="btn-group">
                    <button type="button" class="btn btn-outline-secondary">"Clear log"</button>
                    <PawseButton running_state=running_state />
                </div>
            </div>
            <div class="log-container">
                <EpochSection />
            </div>
        //  <div class="log-container small p-2">
        //    <div
        //      class="log-section mb-3"
        //      v-for="epoch in epochs"
        //      :key="epoch.id"
        //      :ref="el => { if (el) epoch.ref = el as Element }"
        //    >
        //      <div class="border-bottom">
        //        <i18n-t scope="global" :keypath="environment.calendar.epochLabel">
        //          <template #year>
        //            <span class="number">{{ fmt.number(epoch.year) }}</span>
        //          </template>
        //          <template #season>{{ t(epoch.seasonLabel) }}</template>
        //        </i18n-t>
        //      </div>
        //      <div
        //        class="log-event"
        //        v-for="event in epoch.events"
        //        :key="event.id"
        //        :ref="el => { if (el) event.ref = el as Element }"
        //      >
        //        {{ event.text }}
        //      </div>
        //    </div>
        //  </div>
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
