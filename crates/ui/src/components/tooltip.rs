use floating_ui_leptos::{
    use_floating, Flip, FlipOptions, MiddlewareVec, Placement, Shift, ShiftOptions,
    UseFloatingOptions, UseFloatingReturn,
};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

#[slot]
pub struct Target {
    children: Children,
}

#[slot]
pub struct Tooltip {
    children: Children,
}

#[component]
pub fn TooltipContainer(target: Target, tooltip: Tooltip) -> impl IntoView {
    let is_open = RwSignal::new(false);

    let middlewares: MiddlewareVec = vec![
        Box::new(Flip::new(FlipOptions::default())),
        Box::new(Shift::new(ShiftOptions::default())),
    ];
    let target_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();
    let UseFloatingReturn {
        floating_styles, ..
    } = use_floating(
        target_ref,
        floating_ref,
        UseFloatingOptions::default()
            .open(is_open.into())
            .placement(Placement::BottomStart.into())
            .middleware(SendWrapper::new(middlewares).into())
            .while_elements_mounted_auto_update(),
    );

    let target = view! {
        <div
            class="tooltip-target"
            node_ref=target_ref
            on:mouseenter=move |_| is_open.set(true)
            on:mouseleave=move |_| is_open.set(false)
        >
            {(target.children)()}
        </div>
    };

    let tooltip = view! {
        <div
            class="tooltip-content p-2"
            node_ref=floating_ref
            style:position=move || floating_styles.get().style_position()
            style:top=move || floating_styles.get().style_top()
            style:left=move || floating_styles.get().style_left()
            style:transform=move || floating_styles.get().style_transform().unwrap_or_default()
            style:will-change=move || floating_styles.get().style_will_change().unwrap_or_default()
        >
            {(tooltip.children)()}
        </div>
    };

    view! {
        <div class="tooltip-container">
            {target}
            {tooltip}
        </div>
    }
}
