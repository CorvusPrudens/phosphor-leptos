//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "weather"))]
#[component]
pub fn CloudFog(
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
                <path d="M168,232a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16h56A8,8,0,0,1,168,232Zm-40-32a8,8,0,0,0-8-8H72a8,8,0,0,0,0,16h48A8,8,0,0,0,128,200Zm56-8H160a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Zm47.87-96.45a76,76,0,0,0-151.78.73A8.18,8.18,0,0,1,72,104l-.6,0A8.14,8.14,0,0,1,64,95.39a92.48,92.48,0,0,1,2.33-16.51,4,4,0,0,0-5-4.78A52.09,52.09,0,0,0,24,124.36C24.2,153.07,48.12,176,76.84,176H156A76.08,76.08,0,0,0,231.87,95.55Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,100a68,68,0,0,1-68,68H76A44,44,0,1,1,90.2,82.34v.11A68.06,68.06,0,0,1,224,100Z"
        opacity="0.2"
    ></path>
    <path d="M120,208H72a8,8,0,0,1,0-16h48a8,8,0,0,1,0,16Zm64-16H160a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Zm-24,32H104a8,8,0,0,0,0,16h56a8,8,0,0,0,0-16Zm72-124a76.08,76.08,0,0,1-76,76H76A52,52,0,0,1,76,72a53.26,53.26,0,0,1,8.92.76A76.08,76.08,0,0,1,232,100Zm-16,0A60.06,60.06,0,0,0,96,96.46a8,8,0,0,1-16-.92q.21-3.66.77-7.23A38.11,38.11,0,0,0,76,88a36,36,0,0,0,0,72h80A60.07,60.07,0,0,0,216,100Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M120,204H72a4,4,0,0,1,0-8h48a4,4,0,0,1,0,8Zm64-8H160a4,4,0,0,0,0,8h24a4,4,0,0,0,0-8Zm-24,32H104a4,4,0,0,0,0,8h56a4,4,0,0,0,0-8Zm68-128a72.08,72.08,0,0,1-72,72H76A48,48,0,1,1,87.51,77.39,72.08,72.08,0,0,1,228,100Zm-8,0A64.06,64.06,0,0,0,92,96.23a4,4,0,0,1-8-.46,71.63,71.63,0,0,1,1.42-10.65A40,40,0,1,0,76,164h80A64.07,64.07,0,0,0,220,100Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M120,208H72a12,12,0,0,1,0-24h48a12,12,0,0,1,0,24Zm64-24H160a12,12,0,0,0,0,24h24a12,12,0,0,0,0-24Zm-24,36H104a12,12,0,0,0,0,24h56a12,12,0,0,0,0-24ZM232,96a76.08,76.08,0,0,1-76,76H76A52,52,0,1,1,85,68.78,76,76,0,0,1,232,96Zm-24,0a52,52,0,0,0-104,0,12,12,0,0,1-24,0c0-1.24,0-2.48.09-3.71A29.28,29.28,0,0,0,76,92a28,28,0,0,0,0,56h80A52.06,52.06,0,0,0,208,96Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M120,206H72a6,6,0,0,1,0-12h48a6,6,0,0,1,0,12Zm64-12H160a6,6,0,0,0,0,12h24a6,6,0,0,0,0-12Zm-24,32H104a6,6,0,0,0,0,12h56a6,6,0,0,0,0-12Zm70-126a74.09,74.09,0,0,1-74,74H76A50,50,0,1,1,86.2,75,74.08,74.08,0,0,1,230,100Zm-12,0A62.06,62.06,0,0,0,94,96.35a6,6,0,0,1-12-.7,75.84,75.84,0,0,1,1.07-9A38,38,0,1,0,76,162h80A62.07,62.07,0,0,0,218,100Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M120,208H72a8,8,0,0,1,0-16h48a8,8,0,0,1,0,16Zm64-16H160a8,8,0,0,0,0,16h24a8,8,0,0,0,0-16Zm-24,32H104a8,8,0,0,0,0,16h56a8,8,0,0,0,0-16Zm72-124a76.08,76.08,0,0,1-76,76H76A52,52,0,0,1,76,72a53.26,53.26,0,0,1,8.92.76A76.08,76.08,0,0,1,232,100Zm-16,0A60.06,60.06,0,0,0,96,96.46a8,8,0,0,1-16-.92q.21-3.66.77-7.23A38.11,38.11,0,0,0,76,88a36,36,0,0,0,0,72h80A60.07,60.07,0,0,0,216,100Z"></path>
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
