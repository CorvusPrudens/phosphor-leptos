//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map"))]
#[component]
pub fn GlobeStand(
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
                <path d="M56,104a80,80,0,1,1,80,80A80.09,80.09,0,0,1,56,104Zm146.46,69.28A96,96,0,0,1,66.72,37.54,8,8,0,1,0,55.18,26.46,112,112,0,0,0,128,215.71V232H104a8,8,0,0,0,0,16h64a8,8,0,0,0,0-16H144V215.72a111.21,111.21,0,0,0,69.54-30.9,8,8,0,1,0-11.08-11.54Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,104a72,72,0,1,1-72-72A72,72,0,0,1,208,104Z" opacity="0.2"></path>
    <path d="M136,184a80,80,0,1,0-80-80A80.09,80.09,0,0,0,136,184Zm0-144a64,64,0,1,1-64,64A64.07,64.07,0,0,1,136,40Zm77.77,133.5a8,8,0,0,1-.23,11.32A111.21,111.21,0,0,1,144,215.72V232h24a8,8,0,0,1,0,16H104a8,8,0,0,1,0-16h24V215.71A112,112,0,0,1,55.18,26.46,8,8,0,1,1,66.72,37.54,96,96,0,0,0,202.46,173.28,8,8,0,0,1,213.77,173.5Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M136,180a76,76,0,1,0-76-76A76.08,76.08,0,0,0,136,180Zm0-144a68,68,0,1,1-68,68A68.07,68.07,0,0,1,136,36Zm74.89,140.28a4,4,0,0,1-.12,5.65,107.31,107.31,0,0,1-70.77,30V236h28a4,4,0,0,1,0,8H104a4,4,0,0,1,0-8h28V211.92A108,108,0,0,1,58.06,29.23a4,4,0,1,1,5.77,5.54,100,100,0,0,0,141.4,141.39A4,4,0,0,1,210.89,176.28Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M136,176a72,72,0,1,0-72-72A72.08,72.08,0,0,0,136,176Zm0-120a48,48,0,1,1-48,48A48.05,48.05,0,0,1,136,56Zm12,159.35V228h20a12,12,0,0,1,0,24H104a12,12,0,0,1,0-24h20V215.36A112,112,0,0,1,53.88,27.84,12,12,0,1,1,71.47,44.16,88,88,0,0,0,195.84,168.53a12,12,0,1,1,16.32,17.59A111.51,111.51,0,0,1,148,215.35Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M136,182a78,78,0,1,0-78-78A78.09,78.09,0,0,0,136,182Zm0-144a66,66,0,1,1-66,66A66.08,66.08,0,0,1,136,38Zm76.33,136.89a6,6,0,0,1-.17,8.48A109.21,109.21,0,0,1,142,213.83V234h26a6,6,0,0,1,0,12H104a6,6,0,0,1,0-12h26V213.83a110,110,0,0,1-73.38-186,6,6,0,0,1,8.66,8.32A98,98,0,0,0,203.84,174.72,6,6,0,0,1,212.33,174.89Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,184a80,80,0,1,0-80-80A80.09,80.09,0,0,0,136,184Zm0-144a64,64,0,1,1-64,64A64.07,64.07,0,0,1,136,40Zm77.77,133.5a8,8,0,0,1-.23,11.32A111.24,111.24,0,0,1,144,215.72V232h24a8,8,0,0,1,0,16H104a8,8,0,0,1,0-16h24V215.71A112,112,0,0,1,55.18,26.46,8,8,0,1,1,66.72,37.54,96,96,0,0,0,202.46,173.28,8,8,0,0,1,213.77,173.5Z"></path>
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
