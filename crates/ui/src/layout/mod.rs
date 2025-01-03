mod controls;
mod environment;
mod resources;

use leptos::prelude::*;

use controls::ControlsContainer;
use environment::EnvironmentContainer;
use resources::ResourcesContainer;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="app" class="h-100 flex flex-col">
            <header class="bg-gray-100/50 flex flex-row gap-1 px-2 py-1 items-center">
                <div class="order-first flex flex-row gap-1 items-center">
                    <div>"Observable Sorrow"</div>
                    <div class="badge bg-success">
                        <i class="bi bi-droplet"></i>
                        " β "
                    </div>
                </div>
            </header>
            <main class="flex-shrink grid-top-nav-layout gap-0 unscroll-y *:p-2">
                <section class="navigation-area flex justify-center">
                    <ul class="flex flex-wrap gap-2 text-gray-500 dark:text-gray-400">
                        <li>
                            <button type="button" class="inline-block p-1 rounded-md text-white bg-blue-600 active">"Bonfire"</button>
                        </li>
                        <li>
                            <button type="button" class="inline-block p-1 rounded-md hover:text-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-white">"Tab 2"</button>
                        </li>
                    </ul>
                </section>
                <ResourcesContainer />
                <ControlsContainer />
                <EnvironmentContainer />
            </main>
            <footer class="bg-gray-100/50 flex flex-row gap-1 px-2 py-1 justify-end">
                <div>
                    "Observable Sorrow is a clone of "
                    <a href="https://kittensgame.com/web/">"Kittens Game"</a> "."
                </div>
            </footer>
        </div>
    }
}
