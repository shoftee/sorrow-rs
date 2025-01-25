use leptos::component;
use leptos_i18n::t_string;

use sorrow_core::{
    communication::WorkOrderKind,
    state::{buildings::BuildingKind, recipes::CraftingRecipeKind, resources::ResourceKind},
};

use crate::i18n::use_i18n;

#[component]
pub fn ResourceLabel(resource: ResourceKind) -> &'static str {
    let i18n = use_i18n();
    match resource {
        ResourceKind::Catnip => t_string!(i18n, resources.catnip.label),
        ResourceKind::Wood => t_string!(i18n, resources.wood.label),
    }
}

#[component]
pub fn WorkOrderLabel(work_order: WorkOrderKind) -> &'static str {
    let i18n = use_i18n();
    match work_order {
        WorkOrderKind::Construct(building_kind) => match building_kind {
            BuildingKind::CatnipField => t_string!(i18n, buildings.catnip_field.label),
        },
        WorkOrderKind::Craft(crafting_recipe_kind) => match crafting_recipe_kind {
            CraftingRecipeKind::GatherCatnip => t_string!(i18n, bonfire.gather_catnip.label),
            CraftingRecipeKind::RefineCatnip => t_string!(i18n, bonfire.refine_catnip.label),
        },
    }
}
