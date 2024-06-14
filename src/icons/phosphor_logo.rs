//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand"))]
#[component]
pub fn PhosphorLogo(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M152,32H72a8,8,0,0,0-8,8V168a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V176a72,72,0,0,0,0-144ZM136,231.5A64.14,64.14,0,0,1,80.51,176H136Zm0-94L85.68,48H136ZM152,160V48a56,56,0,0,1,0,112Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,104a64,64,0,0,1-64,64h-8V40h8A64,64,0,0,1,216,104ZM72,168h72L72,40Z"
        opacity="0.2"
    ></path>
    <path d="M152,32H72a8,8,0,0,0-8,8V168a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V176a72,72,0,0,0,0-144ZM80,70.54,130.32,160H80Zm56,161A64.14,64.14,0,0,1,80.51,176H136Zm0-94L85.68,48H136ZM152,160V48a56,56,0,0,1,0,112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M152,36H72a4,4,0,0,0-4,4V168a76.08,76.08,0,0,0,76,76,4,4,0,0,0,4-4V172h4a68,68,0,0,0,0-136ZM76,55.27,137.16,164H76Zm64,97.46L78.84,44H140ZM76.13,172H140v63.88A68.1,68.1,0,0,1,76.13,172ZM152,164h-4V44h4a60,60,0,0,1,0,120Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M228,104a76.08,76.08,0,0,0-76-76H72A12,12,0,0,0,60,40V168a84.09,84.09,0,0,0,84,84,12,12,0,0,0,12-12V179.89A76.09,76.09,0,0,0,228,104ZM84,85.81,123.48,156H84Zm48,36.38L92.52,52H132ZM85.22,180H132v46.79A60.18,60.18,0,0,1,85.22,180ZM156,155.83V52.17a52,52,0,0,1,0,103.66Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M152,34H72a6,6,0,0,0-6,6V168a78.09,78.09,0,0,0,78,78,6,6,0,0,0,6-6V174h2a70,70,0,0,0,0-140ZM78,62.91,133.74,162H78Zm60,82.19L82.26,46H138ZM78.28,174H138v59.73A66.1,66.1,0,0,1,78.28,174ZM152,162h-2V46h2a58,58,0,0,1,0,116Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,32H72a8,8,0,0,0-8,8V168a80.09,80.09,0,0,0,80,80,8,8,0,0,0,8-8V176a72,72,0,0,0,0-144ZM80,70.54,130.32,160H80Zm56,66.92L85.68,48H136ZM80.51,176H136v55.5A64.14,64.14,0,0,1,80.51,176ZM152,160V48a56,56,0,0,1,0,112Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
