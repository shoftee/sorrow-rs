use leptos::*;
use sorrow_core::communication::{Command, TimeControl};
use sorrow_core::reactive::State;
use sorrow_core::state::RunningState;

use crate::state::use_command_sink;
use crate::{events::use_keyboard_events, state::use_state_signals};

use crate::conditional::*;
use crate::number_view::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
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
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div>
                "Observable Sorrow is a clone of "
                <a href="https://kittensgame.com/web/">"Kittens Game"</a> "."
            </div>
        </footer>
    }
}

#[component]
pub fn Center() -> impl IntoView {
    let _keyboard_events = use_keyboard_events();

    view! {
        <main class="unscrollable">
            <div class="nav-container">
                <div>"navigation goes here"</div>
                <div class="main-container unscrollable">
                    <div class="col resources-col unscrollable">
                        <ResourcesContainer />
                    </div>
                    <div class="col controls-col unscrollable">
                        <div class="container controls-container">
                            <div class="row">
                                <div class="col">"Controls go here"</div>
                                <div class="col">"Controls go here"</div>
                            </div>
                        </div>
                    </div>
                    <div class="col environment-col unscrollable">
                        <EnvironmentContainer />
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let state_signals = use_state_signals();

    let catnip = Signal::derive(move || state_signals.resource.catnip.get());

    let expanded_rw = create_rw_signal(true);

    view! {
        <div class="resources-container">
            <ul class="list-group resources-list">
                <ResourceExpander expanded=expanded_rw />
                <Conditional>
                    <Main slot condition=expanded_rw>
                        <li class="list-group-item small">"catnip " <DecimalView value=catnip /></li>
                    </Main>
                </Conditional>
            </ul>
        </div>
    }
}

#[component]
fn ResourceExpander(expanded: RwSignal<bool>) -> impl IntoView {
    let collapsed = Signal::derive(move || !expanded.get());

    view! {
        <button
            on:click=move |_| expanded.update(|v| *v = !*v)
            class="list-group-item list-group-item-action expander"
        >
            <div>"Resources"</div>
            <Conditional>
                <Main slot condition=collapsed>
                    <div><i class="bi bi-arrows-expand"></i></div>
                </Main>
            </Conditional>
        </button>
    }
}

#[component]
fn NoResources() -> impl IntoView {
    view! { <li class="list-group-item">"Your paws are empty."</li> }
}

#[component]
pub fn EnvironmentContainer() -> impl IntoView {
    let state = use_state_signals();

    view! {
        <div class="environment-container">"Calendar and History"</div>
        <PawseButton running_state=state.time.running_state />
    }
}

#[component]
fn PawseButton(running_state: State<RunningState>) -> impl IntoView {
    let pawsed = create_memo(move |_| matches!(running_state.get(), RunningState::Paused));

    let command_sink = use_command_sink();
    let toggle = move |_| {
        command_sink.send(if pawsed.get() {
            Command::TimeControl(TimeControl::Start)
        } else {
            Command::TimeControl(TimeControl::Pause)
        });
    };

    view! {
        <button type="button" class="btn btn-outline-secondary" class:active=pawsed on:click=toggle>{
            move || if pawsed.get() { "Unpawse" } else { "Pawse" }
        }</button>
    }
}
