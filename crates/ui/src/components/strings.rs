use leptos::component;
use leptos_i18n::t_string;

use sorrow_core::state::resources::ResourceKind;

use crate::i18n::use_i18n;

#[component]
pub fn ResourceLabel(resource: ResourceKind) -> &'static str {
    let i18n = use_i18n();
    match resource {
        ResourceKind::Catnip => t_string!(i18n, resources.catnip.label),
        ResourceKind::Wood => t_string!(i18n, resources.wood.label),
    }
}
