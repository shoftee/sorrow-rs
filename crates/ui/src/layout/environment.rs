use leptos::prelude::*;
use leptos_i18n::*;

use sorrow_core::{
    communication::*,
    state::{calendar::SeasonKind, time::RunningState},
};

use crate::{
    endpoint::use_endpoint,
    i18n::use_i18n,
    store::{use_global_store, CalendarStoreFields, GlobalStoreFields},
};

#[component]
pub fn EnvironmentContainer() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="environment-area unscroll-y flex flex-col gap-2">
            <Calendar />
            <div>{ t!(i18n, game.blurb) }</div>
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
    let i18n = use_i18n();
    let store = use_global_store().calendar();

    let day = Memo::new(move |_| store.day().get());
    let season = Memo::new(move |_| season_label(i18n, store.season().get()));
    let year = Memo::new(move |_| store.year().get());

    view! {
        <div>{ t!(i18n, calendar.full.no_weather, year, season, day) }</div>
    }
}

#[component]
fn ClearLog() -> impl IntoView {
    let i18n = use_i18n_scoped!(game.control);

    let label = Signal::derive(move || t_string!(i18n, clear_log));

    view! {
        <button type="button" class="btn padded rounded">{ label }</button>
    }
}

#[component]
fn PawseButton() -> impl IntoView {
    let i18n = use_i18n();

    let store = use_global_store();
    let endpoint = use_endpoint();

    let running_state = Memo::new(move |_| store.running_state().get());
    let pawsed = Memo::new(move |_| matches!(running_state.get(), RunningState::Paused));

    let button_label = Signal::derive(move || {
        if pawsed.get() {
            t_string!(i18n, game.control.unpawse)
        } else {
            t_string!(i18n, game.control.pawse)
        }
    });

    let button_intent = Signal::derive(move || {
        if pawsed.get() {
            Intent::TimeControl(TimeControl::Start)
        } else {
            Intent::TimeControl(TimeControl::Pause)
        }
    });

    view! {
        <button type="button"
            class="btn padded rounded"
            class:active=pawsed
            on:click=move |_| endpoint.send(button_intent.get())
        >
            {move || button_label.get()}
        </button>
    }
}

#[component]
fn EpochSection() -> impl IntoView {
    let i18n = use_i18n();

    // read_untracked because Epoch sections designate a snapshot of time and should not change.
    let calendar = use_global_store().calendar().read_untracked();

    // Need to react to locale changes for season, though.
    let season = Signal::derive({
        let season = calendar.season;
        move || season_label(i18n, season)
    });
    let year = calendar.year;

    let formatted_date =
        Signal::derive(move || t_string!(i18n, calendar.epoch.full, year, season = season.get()));

    view! {
        <div>
            <div class="border-b border-solid border-neutral-400">{ formatted_date }</div>
            <ul class="text-xs list-disc">
                <li>"Test Event"</li>
            </ul>
        </div>
    }
}

fn season_label(
    i18n: leptos_i18n::I18nContext<crate::i18n::Locale>,
    season: SeasonKind,
) -> &'static str {
    match season {
        SeasonKind::Spring => t_string!(i18n, environment.seasons.spring.label),
        SeasonKind::Summer => t_string!(i18n, environment.seasons.summer.label),
        SeasonKind::Autumn => t_string!(i18n, environment.seasons.autumn.label),
        SeasonKind::Winter => t_string!(i18n, environment.seasons.winter.label),
    }
}
