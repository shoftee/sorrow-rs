mod controls;
mod environment;
mod resources;

use leptos::prelude::*;
use leptos_i18n::*;

use controls::ControlsContainer;
use environment::EnvironmentContainer;
use resources::ResourcesContainer;
use sorrow_core::state::{
    ui::{NavigationNodeId, NodeId},
    KeyIter,
};

use crate::{
    store::{use_global_store, GlobalStoreFields, UiStateStoreFields},
    use_i18n, Locale,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="app" class="h-100 flex flex-col">
            <Header />
            <main class="flex-shrink grid-top-nav-layout gap-0 unscroll-y *:p-2">
                <Navigation active=NavigationNodeId::Bonfire />
                <ResourcesContainer />
                <ControlsContainer />
                <EnvironmentContainer />
            </main>
            <Footer />
        </div>
    }
}

#[component]
fn Navigation(#[prop(into)] active: Signal<NavigationNodeId>) -> impl IntoView {
    let ui_store = use_global_store().ui();
    let tabs = Signal::derive(move || {
        NavigationNodeId::key_iter()
            .filter(|id| {
                ui_store
                    .read_untracked()
                    .get(&NodeId::Navigation(*id))
                    .unwrap()
                    .visible()
                    .get()
            })
            .collect::<Vec<_>>()
    });

    let is_single = Memo::new(move |_| tabs.get().iter().take(2).count() < 2);

    let i18n = use_i18n();
    view! {
        <section class="navigation-area flex justify-center">
            <ul class="flex flex-wrap gap-2">
                <For each={move || tabs.get()} key={|id| *id} let:child>
                    <li>
                        <button type="button"
                            class="btn"
                            class:active={ move || active.get() == child }
                            prop:disabled=is_single
                        >
                            { nav_label(i18n, child) }
                        </button>
                    </li>
                </For>
            </ul>
        </section>
    }
}

fn nav_label(i18n: I18nContext<Locale>, id: NavigationNodeId) -> &'static str {
    match id {
        NavigationNodeId::Bonfire => t_string!(i18n, sections.bonfire.label),
    }
}

#[component]
fn Header() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <header class="bg-gray-100/50 flex flex-row gap-1 px-2 py-1 items-center">
            <div class="order-first flex flex-row gap-1 items-center">
                <div>{ t!(i18n, game.title) }</div>
                <div class="badge bg-success">
                    <i class="bi bi-droplet"></i>
                    " β "
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <footer class="bg-gray-100/50 flex flex-row gap-1 px-2 py-1 justify-end">
            <div>
                { t!(i18n, game.dedication, <link> = <a href="https://kittensgame.com/web/" />) }
            </div>
        </footer>
    }
}
