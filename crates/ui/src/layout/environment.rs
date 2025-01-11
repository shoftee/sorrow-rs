use leptos::prelude::*;
use sorrow_core::{communication::*, state::time::RunningState};

use crate::{
    components::Button,
    state::{use_global_store, CalendarStoreFields, GlobalStoreStoreFields},
};

#[component]
pub fn EnvironmentContainer() -> impl IntoView {
    view! {
        <section class="environment-area unscroll-y flex flex-col gap-2">
            <Calendar />
            <div>"You are a kitten in a catnip forest."</div>
            <div class="flex flex-row gap-2 *:flex-auto">
                <ClearLog />
                <PawseButton />
            </div>
            <div class="overflow-y-hidden flex-grow flex flex-col text-sm fade-down-to-transparent space-y-4">
                <EpochSection />
            </div>
        </section>
    }
}

#[component]
fn Calendar() -> impl IntoView {
    let store = use_global_store().calendar();

    let day = Memo::new(move |_| store.day().get());
    let season = Memo::new(move |_| {
        use sorrow_core::state::calendar::SeasonKind::*;

        match store.season().get() {
            Spring => "Spring",
            Summer => "Summer",
            Autumn => "Autumn",
            Winter => "Winter",
        }
    });
    let year = Memo::new(move |_| store.year().get());

    view! {
        <div>"Year "{year}" – "{season}", day "{day}</div>
    }
}

#[component]
fn ClearLog() -> impl IntoView {
    view! {
        <button type="button" class="btn">"Clear log"</button>
    }
}

#[component]
fn PawseButton() -> impl IntoView {
    let store = use_global_store();
    let running_state = Memo::new(move |_| store.running_state().get());
    let pawsed = Memo::new(move |_| matches!(running_state.get(), RunningState::Paused));

    let new_intent = Signal::derive(move || {
        if pawsed.get() {
            Intent::TimeControl(TimeControl::Start)
        } else {
            Intent::TimeControl(TimeControl::Pause)
        }
    });

    view! {
        <Button class:active=pawsed intent=new_intent>{
            move || if pawsed.get() { "Unpawse" } else { "Pawse" }
        }</Button>
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
