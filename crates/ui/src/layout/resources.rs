use leptos::prelude::*;
use leptos_i18n::*;
use reactive_stores::Store;

use sorrow_core::state::resources::Kind;
use sorrow_core::state::ui::{NodeId, ResourceNodeId};
use sorrow_core::state::KeyIter;

use crate::components::{conditional::*, numbers::DecimalView};
use crate::formatter::ShowSign;
use crate::i18n::use_i18n;
use crate::store::{use_global_store, GlobalStoreFields, ResourceStoreFields, UiStateStoreFields};

#[component]
pub fn ResourcesContainer() -> impl IntoView {
    let i18n = use_i18n();
    let ui_store = use_global_store().ui();
    let resources_store = use_global_store().resources();

    let resources = Signal::derive(move || {
        ResourceNodeId::key_iter()
            .filter_map(|id| {
                if ui_store
                    .read_untracked()
                    .get(&NodeId::Resources(id))
                    .unwrap()
                    .visible()
                    .get()
                {
                    Some(*resources_store.read_untracked().get(&id.into()).unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });
    let has_resources = Memo::new(move |_| !resources.get().is_empty());

    let expanded_rw = RwSignal::new(true);
    let is_collapsed = Signal::derive(move || !expanded_rw.get());

    view! {
        <section class="resources-area unscroll-y">
            <div class="resources-container">
                <Conditional>
                    <Main slot condition=has_resources>
                        <ul class="list-group resources-list">
                            <button
                                on:click=move |_| expanded_rw.update(|v| *v = !*v)
                                class="list-group-item list-group-item-action expander"
                            >
                                <div>{ t_string!(i18n, resources.section.label) }</div>
                                <Conditional>
                                    <Main slot condition=is_collapsed>
                                        <div><i class="bi bi-arrows-expand"></i></div>
                                    </Main>
                                </Conditional>
                            </button>
                            <Conditional>
                                <Main slot condition=expanded_rw>
                                    <For each={move || resources.get()} key=|item| item.kind().get() let:child>
                                        <ResourceItem item=child />
                                    </For>
                                </Main>
                            </Conditional>
                        </ul>
                    </Main>
                    <Fallback slot>
                        <li class="list-group-item">{ t_string!(i18n, resources.section.empty) }</li>
                    </Fallback>
                </Conditional>
            </div>
        </section>
    }
}

#[component]
fn ResourceItem(#[prop(into)] item: Store<crate::store::Resource>) -> impl IntoView {
    let i18n = use_i18n();

    let amount = Memo::new(move |_| item.amount().get());
    let delta = Memo::new(move |_| item.delta().get());

    view! {
        <li class="list-group-item small">
            { resource_label(i18n, item.kind().get()) }
            " "
            <DecimalView value=amount />
            " "
            <DecimalView value=delta show_sign=ShowSign::Always />
        </li>
    }
}

fn resource_label(
    i18n: leptos_i18n::I18nContext<crate::i18n::Locale>,
    kind: sorrow_core::state::resources::Kind,
) -> &'static str {
    match kind {
        Kind::Catnip => t_string!(i18n, resources.catnip.label),
        Kind::Wood => t_string!(i18n, resources.wood.label),
    }
}
