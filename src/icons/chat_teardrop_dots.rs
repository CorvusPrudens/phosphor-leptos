//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication"))]
#[component]
pub fn ChatTeardropDots(
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
                <path d="M132,24A100.11,100.11,0,0,0,32,124v84a16,16,0,0,0,16,16h84a100,100,0,0,0,0-200ZM88,140a12,12,0,1,1,12-12A12,12,0,0,1,88,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,132,140Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,176,140Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,124h0a92,92,0,0,1-92,92H48a8,8,0,0,1-8-8V124a92,92,0,0,1,92-92h0A92,92,0,0,1,224,124Z"
        opacity="0.2"
    ></path>
    <path d="M132,24A100.11,100.11,0,0,0,32,124v84a16,16,0,0,0,16,16h84a100,100,0,0,0,0-200Zm0,184H48V124a84,84,0,1,1,84,84Zm12-80a12,12,0,1,1-12-12A12,12,0,0,1,144,128Zm-44,0a12,12,0,1,1-12-12A12,12,0,0,1,100,128Zm88,0a12,12,0,1,1-12-12A12,12,0,0,1,188,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M140,128a8,8,0,1,1-8-8A8,8,0,0,1,140,128Zm-52-8a8,8,0,1,0,8,8A8,8,0,0,0,88,120Zm88,0a8,8,0,1,0,8,8A8,8,0,0,0,176,120Zm52,4a96.11,96.11,0,0,1-96,96H48a12,12,0,0,1-12-12V124a96,96,0,0,1,192,0Zm-8,0a88,88,0,0,0-176,0v84a4,4,0,0,0,4,4h84A88.1,88.1,0,0,0,220,124Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M132,20A104.11,104.11,0,0,0,28,124v84a20,20,0,0,0,20,20h84a104,104,0,0,0,0-208Zm0,184H52V124a80,80,0,1,1,80,80Zm-8-76a16,16,0,1,1-16-16A16,16,0,0,1,124,128Zm48,0a16,16,0,1,1-16-16A16,16,0,0,1,172,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M142,128a10,10,0,1,1-10-10A10,10,0,0,1,142,128ZM88,118a10,10,0,1,0,10,10A10,10,0,0,0,88,118Zm88,0a10,10,0,1,0,10,10A10,10,0,0,0,176,118Zm54,6a98.11,98.11,0,0,1-98,98H48a14,14,0,0,1-14-14V124a98,98,0,0,1,196,0Zm-12,0a86,86,0,0,0-172,0v84a2,2,0,0,0,2,2h84A86.1,86.1,0,0,0,218,124Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M132,24A100.11,100.11,0,0,0,32,124v84a16,16,0,0,0,16,16h84a100,100,0,0,0,0-200Zm0,184H48V124a84,84,0,1,1,84,84Zm12-80a12,12,0,1,1-12-12A12,12,0,0,1,144,128Zm-44,0a12,12,0,1,1-12-12A12,12,0,0,1,100,128Zm88,0a12,12,0,1,1-12-12A12,12,0,0,1,188,128Z"></path>
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
