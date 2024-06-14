//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowElbowLeftUp(
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
                <path d="M240,192a8,8,0,0,1-8,8H88a8,8,0,0,1-8-8V104H40a8,8,0,0,1-5.66-13.66l48-48a8,8,0,0,1,11.32,0l48,48A8,8,0,0,1,136,104H96v80H232A8,8,0,0,1,240,192Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M136,96H40L88,48Z" opacity="0.2"></path>
    <path d="M232,184H96V104h40a8,8,0,0,0,5.66-13.66l-48-48a8,8,0,0,0-11.32,0l-48,48A8,8,0,0,0,40,104H80v88a8,8,0,0,0,8,8H232a8,8,0,0,0,0-16ZM88,59.31,116.69,88H59.31Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M236,192a4,4,0,0,1-4,4H88a4,4,0,0,1-4-4V57.66L42.83,98.83a4,4,0,0,1-5.66-5.66l48-48a4,4,0,0,1,5.66,0l48,48a4,4,0,0,1-5.66,5.66L92,57.66V188H232A4,4,0,0,1,236,192Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,192a12,12,0,0,1-12,12H88a12,12,0,0,1-12-12V77L48.49,104.49a12,12,0,0,1-17-17l48-48a12,12,0,0,1,17,0l48,48a12,12,0,0,1-17,17L100,77V180H232A12,12,0,0,1,244,192Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M238,192a6,6,0,0,1-6,6H88a6,6,0,0,1-6-6V62.49L44.24,100.24a6,6,0,0,1-8.48-8.48l48-48a6,6,0,0,1,8.48,0l48,48a6,6,0,1,1-8.48,8.48L94,62.49V186H232A6,6,0,0,1,238,192Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,192a8,8,0,0,1-8,8H88a8,8,0,0,1-8-8V67.31L45.66,101.66A8,8,0,0,1,34.34,90.34l48-48a8,8,0,0,1,11.32,0l48,48a8,8,0,0,1-11.32,11.32L96,67.31V184H232A8,8,0,0,1,240,192Z"></path>
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
