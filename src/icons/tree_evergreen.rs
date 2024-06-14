//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn TreeEvergreen(
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
                <path d="M231.19,195.51A8,8,0,0,1,224,200H136v40a8,8,0,0,1-16,0V200H32a8,8,0,0,1-6.31-12.91l46-59.09H48a8,8,0,0,1-6.34-12.88l80-104a8,8,0,0,1,12.68,0l80,104A8,8,0,0,1,208,128H184.36l45.95,59.09A8,8,0,0,1,231.19,195.51Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M32,192l56-72H48L128,16l80,104H168l56,72Z" opacity="0.2"></path>
    <path d="M230.31,187.09,184.36,128H208a8,8,0,0,0,6.34-12.88l-80-104a8,8,0,0,0-12.68,0l-80,104A8,8,0,0,0,48,128H71.64L25.69,187.09A8,8,0,0,0,32,200h88v40a8,8,0,0,0,16,0V200h88a8,8,0,0,0,6.31-12.91ZM48.36,184l46-59.09A8,8,0,0,0,88,112H64.25L128,29.12,191.75,112H168a8,8,0,0,0-6.31,12.91L207.64,184Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M227.16,189.54l-51-65.54H208a4,4,0,0,0,3.17-6.44l-80-104a4,4,0,0,0-6.34,0l-80,104A4,4,0,0,0,48,124H79.82l-51,65.54A4,4,0,0,0,32,196h92v44a4,4,0,0,0,8,0V196h92a4,4,0,0,0,3.16-6.46ZM40.18,188l51-65.54A4,4,0,0,0,88,116H56.12L128,22.56,199.88,116H168a4,4,0,0,0-3.16,6.46l51,65.54Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M233.47,184.63,192.54,132H208a12,12,0,0,0,9.51-19.32l-80-104a12,12,0,0,0-19,0l-80,104A12,12,0,0,0,48,132H63.46L22.53,184.63A12,12,0,0,0,32,204h84v36a12,12,0,0,0,24,0V204h84a12,12,0,0,0,9.47-19.37ZM56.54,180l40.93-52.63A12,12,0,0,0,88,108H72.37L128,35.68,183.63,108H168a12,12,0,0,0-9.47,19.37L199.46,180Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.74,188.32,180.27,126H208a6,6,0,0,0,4.76-9.66l-80-104a6,6,0,0,0-9.52,0l-80,104A6,6,0,0,0,48,126H75.73L27.26,188.32A6,6,0,0,0,32,198h90v42a6,6,0,0,0,12,0V198h90a6,6,0,0,0,4.74-9.68ZM44.27,186l48.47-62.32A6,6,0,0,0,88,114H60.19L128,25.84,195.81,114H168a6,6,0,0,0-4.74,9.68L211.73,186Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M230.32,187.09l-46-59.09H208a8,8,0,0,0,6.34-12.88l-80-104a8,8,0,0,0-12.68,0l-80,104A8,8,0,0,0,48,128H71.64l-46,59.09A8,8,0,0,0,32,200h88v40a8,8,0,0,0,16,0V200h88a8,8,0,0,0,6.32-12.91ZM48.36,184l46-59.09A8,8,0,0,0,88,112H64.25L128,29.12,191.75,112H168a8,8,0,0,0-6.31,12.91L207.64,184Z"></path>
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
