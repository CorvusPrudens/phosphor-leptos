//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "system"))]
#[component]
pub fn Crosshair(
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
                <path d="M232,120h-8.34A96.14,96.14,0,0,0,136,32.34V24a8,8,0,0,0-16,0v8.34A96.14,96.14,0,0,0,32.34,120H24a8,8,0,0,0,0,16h8.34A96.14,96.14,0,0,0,120,223.66V232a8,8,0,0,0,16,0v-8.34A96.14,96.14,0,0,0,223.66,136H232a8,8,0,0,0,0-16Zm-32,16h7.6A80.15,80.15,0,0,1,136,207.6V200a8,8,0,0,0-16,0v7.6A80.15,80.15,0,0,1,48.4,136H56a8,8,0,0,0,0-16H48.4A80.15,80.15,0,0,1,120,48.4V56a8,8,0,0,0,16,0V48.4A80.15,80.15,0,0,1,207.6,120H200a8,8,0,0,0,0,16Zm-32-8a40,40,0,1,1-40-40A40,40,0,0,1,168,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M160,128a32,32,0,1,1-32-32A32,32,0,0,1,160,128Z" opacity="0.2"></path>
    <path d="M232,120h-8.34A96.14,96.14,0,0,0,136,32.34V24a8,8,0,0,0-16,0v8.34A96.14,96.14,0,0,0,32.34,120H24a8,8,0,0,0,0,16h8.34A96.14,96.14,0,0,0,120,223.66V232a8,8,0,0,0,16,0v-8.34A96.14,96.14,0,0,0,223.66,136H232a8,8,0,0,0,0-16Zm-96,87.6V200a8,8,0,0,0-16,0v7.6A80.15,80.15,0,0,1,48.4,136H56a8,8,0,0,0,0-16H48.4A80.15,80.15,0,0,1,120,48.4V56a8,8,0,0,0,16,0V48.4A80.15,80.15,0,0,1,207.6,120H200a8,8,0,0,0,0,16h7.6A80.15,80.15,0,0,1,136,207.6ZM128,88a40,40,0,1,0,40,40A40,40,0,0,0,128,88Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,152Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,124H219.91A92.13,92.13,0,0,0,132,36.09V24a4,4,0,0,0-8,0V36.09A92.13,92.13,0,0,0,36.09,124H24a4,4,0,0,0,0,8H36.09A92.13,92.13,0,0,0,124,219.91V232a4,4,0,0,0,8,0V219.91A92.13,92.13,0,0,0,219.91,132H232a4,4,0,0,0,0-8ZM132,211.9V200a4,4,0,0,0-8,0v11.9A84.11,84.11,0,0,1,44.1,132H56a4,4,0,0,0,0-8H44.1A84.11,84.11,0,0,1,124,44.1V56a4,4,0,0,0,8,0V44.1A84.11,84.11,0,0,1,211.9,124H200a4,4,0,0,0,0,8h11.9A84.11,84.11,0,0,1,132,211.9ZM128,92a36,36,0,1,0,36,36A36,36,0,0,0,128,92Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,128,156Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232,116h-4.72A100.21,100.21,0,0,0,140,28.72V24a12,12,0,0,0-24,0v4.72A100.21,100.21,0,0,0,28.72,116H24a12,12,0,0,0,0,24h4.72A100.21,100.21,0,0,0,116,227.28V232a12,12,0,0,0,24,0v-4.72A100.21,100.21,0,0,0,227.28,140H232a12,12,0,0,0,0-24Zm-92,87v-3a12,12,0,0,0-24,0v3a76.15,76.15,0,0,1-63-63h3a12,12,0,0,0,0-24H53a76.15,76.15,0,0,1,63-63v3a12,12,0,0,0,24,0V53a76.15,76.15,0,0,1,63,63h-3a12,12,0,0,0,0,24h3A76.15,76.15,0,0,1,140,203ZM128,84a44,44,0,1,0,44,44A44.05,44.05,0,0,0,128,84Zm0,64a20,20,0,1,1,20-20A20,20,0,0,1,128,148Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,122H221.8A94.13,94.13,0,0,0,134,34.2V24a6,6,0,0,0-12,0V34.2A94.13,94.13,0,0,0,34.2,122H24a6,6,0,0,0,0,12H34.2A94.13,94.13,0,0,0,122,221.8V232a6,6,0,0,0,12,0V221.8A94.13,94.13,0,0,0,221.8,134H232a6,6,0,0,0,0-12Zm-98,87.76V200a6,6,0,0,0-12,0v9.76A82.09,82.09,0,0,1,46.24,134H56a6,6,0,0,0,0-12H46.24A82.09,82.09,0,0,1,122,46.24V56a6,6,0,0,0,12,0V46.24A82.09,82.09,0,0,1,209.76,122H200a6,6,0,0,0,0,12h9.76A82.09,82.09,0,0,1,134,209.76ZM128,90a38,38,0,1,0,38,38A38,38,0,0,0,128,90Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,128,154Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,120h-8.34A96.14,96.14,0,0,0,136,32.34V24a8,8,0,0,0-16,0v8.34A96.14,96.14,0,0,0,32.34,120H24a8,8,0,0,0,0,16h8.34A96.14,96.14,0,0,0,120,223.66V232a8,8,0,0,0,16,0v-8.34A96.14,96.14,0,0,0,223.66,136H232a8,8,0,0,0,0-16Zm-96,87.6V200a8,8,0,0,0-16,0v7.6A80.15,80.15,0,0,1,48.4,136H56a8,8,0,0,0,0-16H48.4A80.15,80.15,0,0,1,120,48.4V56a8,8,0,0,0,16,0V48.4A80.15,80.15,0,0,1,207.6,120H200a8,8,0,0,0,0,16h7.6A80.15,80.15,0,0,1,136,207.6ZM128,88a40,40,0,1,0,40,40A40,40,0,0,0,128,88Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,152Z"></path>
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
