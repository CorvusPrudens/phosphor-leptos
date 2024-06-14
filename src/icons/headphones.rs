//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "objects"))]
#[component]
pub fn Headphones(
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
                <path d="M232,128v56a24,24,0,0,1-24,24H192a24,24,0,0,1-24-24V144a24,24,0,0,1,24-24h23.65a87.71,87.71,0,0,0-87-80H128a88,88,0,0,0-87.64,80H64a24,24,0,0,1,24,24v40a24,24,0,0,1-24,24H48a24,24,0,0,1-24-24V128A104.11,104.11,0,0,1,201.89,54.66,103.41,103.41,0,0,1,232,128Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M80,144v40a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V128H64A16,16,0,0,1,80,144Zm112-16a16,16,0,0,0-16,16v40a16,16,0,0,0,16,16h16a16,16,0,0,0,16-16V128Z"
        opacity="0.2"
    ></path>
    <path d="M201.89,54.66A104.08,104.08,0,0,0,24,128v56a24,24,0,0,0,24,24H64a24,24,0,0,0,24-24V144a24,24,0,0,0-24-24H40.36A88,88,0,0,1,128,40h.67a87.71,87.71,0,0,1,87,80H192a24,24,0,0,0-24,24v40a24,24,0,0,0,24,24h16a24,24,0,0,0,24-24V128A103.41,103.41,0,0,0,201.89,54.66ZM64,136a8,8,0,0,1,8,8v40a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V136Zm152,48a8,8,0,0,1-8,8H192a8,8,0,0,1-8-8V144a8,8,0,0,1,8-8h24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M199.05,57.48A100.07,100.07,0,0,0,28,128v56a20,20,0,0,0,20,20H64a20,20,0,0,0,20-20V144a20,20,0,0,0-20-20H36.08A92,92,0,0,1,128,36h.7a91.75,91.75,0,0,1,91.22,88H192a20,20,0,0,0-20,20v40a20,20,0,0,0,20,20h16a20,20,0,0,0,20-20V128A99.43,99.43,0,0,0,199.05,57.48ZM64,132a12,12,0,0,1,12,12v40a12,12,0,0,1-12,12H48a12,12,0,0,1-12-12V132Zm156,52a12,12,0,0,1-12,12H192a12,12,0,0,1-12-12V144a12,12,0,0,1,12-12h28Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M204.73,51.85A108.07,108.07,0,0,0,20,128v56a28,28,0,0,0,28,28H64a28,28,0,0,0,28-28V144a28,28,0,0,0-28-28H44.84A84.05,84.05,0,0,1,128,44h.64a83.7,83.7,0,0,1,82.52,72H192a28,28,0,0,0-28,28v40a28,28,0,0,0,28,28h16a28,28,0,0,0,28-28V128A107.34,107.34,0,0,0,204.73,51.85ZM64,140a4,4,0,0,1,4,4v40a4,4,0,0,1-4,4H48a4,4,0,0,1-4-4V140Zm148,44a4,4,0,0,1-4,4H192a4,4,0,0,1-4-4V144a4,4,0,0,1,4-4h20Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200.47,56.07A101.37,101.37,0,0,0,128.77,26H128A102,102,0,0,0,26,128v56a22,22,0,0,0,22,22H64a22,22,0,0,0,22-22V144a22,22,0,0,0-22-22H38.2A90.12,90.12,0,0,1,192,64.52,89.41,89.41,0,0,1,217.81,122H192a22,22,0,0,0-22,22v40a22,22,0,0,0,22,22h16a22,22,0,0,0,22-22V128A101.44,101.44,0,0,0,200.47,56.07ZM64,134a10,10,0,0,1,10,10v40a10,10,0,0,1-10,10H48a10,10,0,0,1-10-10V134Zm154,50a10,10,0,0,1-10,10H192a10,10,0,0,1-10-10V144a10,10,0,0,1,10-10h26Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M201.89,54.66A103.43,103.43,0,0,0,128.79,24H128A104,104,0,0,0,24,128v56a24,24,0,0,0,24,24H64a24,24,0,0,0,24-24V144a24,24,0,0,0-24-24H40.36A88,88,0,0,1,128,40h.67a87.71,87.71,0,0,1,87,80H192a24,24,0,0,0-24,24v40a24,24,0,0,0,24,24h16a24,24,0,0,0,24-24V128A103.41,103.41,0,0,0,201.89,54.66ZM64,136a8,8,0,0,1,8,8v40a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V136Zm152,48a8,8,0,0,1-8,8H192a8,8,0,0,1-8-8V144a8,8,0,0,1,8-8h24Z"></path>
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
