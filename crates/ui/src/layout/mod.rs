mod controls;
mod environment;
mod resources;

use leptos::prelude::*;

use crate::events::use_keyboard_events;

use controls::ControlsContainer;
use environment::EnvironmentContainer;
use resources::ResourcesContainer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="app">
            <div class="app-container">
                <Header/>
                <Center/>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
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
fn Footer() -> impl IntoView {
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
fn Center() -> impl IntoView {
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
                        <ControlsContainer />
                    </div>
                    <div class="col environment-col unscrollable">
                        <EnvironmentContainer />
                    </div>
                </div>
            </div>
        </main>
    }
}
