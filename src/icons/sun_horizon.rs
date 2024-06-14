//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature", feature = "weather"))]
#[component]
pub fn SunHorizon(
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
                <path d="M248,160a8,8,0,0,1-8,8H16a8,8,0,0,1,0-16H56.45a73.54,73.54,0,0,1-.45-8,72,72,0,0,1,144,0,73.54,73.54,0,0,1-.45,8H240A8,8,0,0,1,248,160Zm-40,32H48a8,8,0,0,0,0,16H208a8,8,0,0,0,0-16ZM80.84,59.58a8,8,0,0,0,14.32-7.16l-8-16a8,8,0,0,0-14.32,7.16ZM20.42,103.16l16,8a8,8,0,1,0,7.16-14.31l-16-8a8,8,0,1,0-7.16,14.31ZM216,112a8,8,0,0,0,3.57-.84l16-8a8,8,0,1,0-7.16-14.31l-16,8A8,8,0,0,0,216,112ZM164.42,63.16a8,8,0,0,0,10.74-3.58l8-16a8,8,0,0,0-14.32-7.16l-8,16A8,8,0,0,0,164.42,63.16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,144a64.33,64.33,0,0,1-2,16H66a64,64,0,1,1,126-16Z" opacity="0.2"></path>
    <path d="M240,152H199.55a73.54,73.54,0,0,0,.45-8,72,72,0,0,0-144,0,73.54,73.54,0,0,0,.45,8H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM72,144a56,56,0,1,1,111.41,8H72.59A56.13,56.13,0,0,1,72,144Zm144,56a8,8,0,0,1-8,8H48a8,8,0,0,1,0-16H208A8,8,0,0,1,216,200ZM72.84,43.58a8,8,0,0,1,14.32-7.16l8,16a8,8,0,0,1-14.32,7.16Zm-56,48.84a8,8,0,0,1,10.74-3.57l16,8a8,8,0,0,1-7.16,14.31l-16-8A8,8,0,0,1,16.84,92.42Zm192,15.16a8,8,0,0,1,3.58-10.73l16-8a8,8,0,1,1,7.16,14.31l-16,8a8,8,0,0,1-10.74-3.58Zm-48-55.16,8-16a8,8,0,0,1,14.32,7.16l-8,16a8,8,0,1,1-14.32-7.16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,156H194.94A68,68,0,1,0,60,144a68.73,68.73,0,0,0,1.06,12H16a4,4,0,0,0,0,8H240a4,4,0,0,0,0-8ZM68,144a60,60,0,1,1,118.79,12H69.21A60.16,60.16,0,0,1,68,144Zm144,56a4,4,0,0,1-4,4H48a4,4,0,0,1,0-8H208A4,4,0,0,1,212,200ZM76.42,41.79a4,4,0,0,1,7.16-3.58l8,16a4,4,0,0,1-7.16,3.58Zm-56,52.42a4,4,0,0,1,5.37-1.79l16,8a4,4,0,0,1-3.58,7.16l-16-8A4,4,0,0,1,20.42,94.21Zm192,11.58a4,4,0,0,1,1.79-5.37l16-8a4,4,0,1,1,3.58,7.16l-16,8a4,4,0,0,1-5.37-1.79Zm-48-51.58,8-16a4,4,0,1,1,7.16,3.58l-8,16a4,4,0,0,1-7.16-3.58Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,148H203.89c.07-1.33.11-2.66.11-4a76,76,0,0,0-152,0c0,1.34,0,2.67.11,4H16a12,12,0,0,0,0,24H240a12,12,0,0,0,0-24ZM76,144a52,52,0,0,1,104,0c0,1.34-.07,2.67-.17,4H76.17C76.07,146.67,76,145.34,76,144Zm144,56a12,12,0,0,1-12,12H48a12,12,0,0,1,0-24H208A12,12,0,0,1,220,200ZM12.62,92.21a12,12,0,0,1,15.17-7.59l12,4a12,12,0,1,1-7.58,22.77l-12-4A12,12,0,0,1,12.62,92.21Zm56-48.41a12,12,0,1,1,22.76-7.59l4,12A12,12,0,1,1,72.62,55.8Zm140,60a12,12,0,0,1,7.59-15.18l12-4a12,12,0,0,1,7.58,22.77l-12,4a12,12,0,0,1-15.17-7.59Zm-48-55.59,4-12a12,12,0,1,1,22.76,7.59l-4,12a12,12,0,1,1-22.76-7.59Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,154H197.28a70.91,70.91,0,0,0,.72-10,70,70,0,0,0-140,0,70.91,70.91,0,0,0,.72,10H16a6,6,0,0,0,0,12H240a6,6,0,0,0,0-12ZM70,144a58,58,0,1,1,115.13,10H70.87A58.63,58.63,0,0,1,70,144Zm144,56a6,6,0,0,1-6,6H48a6,6,0,0,1,0-12H208A6,6,0,0,1,214,200ZM74.63,42.69a6,6,0,0,1,10.74-5.37l8,16a6,6,0,0,1-10.74,5.36Zm-56,50.63a6,6,0,0,1,8.05-2.69l16,8a6,6,0,0,1-5.36,10.74l-16-8A6,6,0,0,1,18.63,93.32Zm192,13.36a6,6,0,0,1,2.69-8.05l16-8a6,6,0,1,1,5.36,10.74l-16,8a6,6,0,0,1-8.05-2.69Zm-48-53.36,8-16a6,6,0,0,1,10.74,5.37l-8,16a6,6,0,1,1-10.74-5.36Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,152H199.55a73.54,73.54,0,0,0,.45-8,72,72,0,0,0-144,0,73.54,73.54,0,0,0,.45,8H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM72,144a56,56,0,1,1,111.41,8H72.59A56.13,56.13,0,0,1,72,144Zm144,56a8,8,0,0,1-8,8H48a8,8,0,0,1,0-16H208A8,8,0,0,1,216,200ZM72.84,43.58a8,8,0,0,1,14.32-7.16l8,16a8,8,0,0,1-14.32,7.16Zm-56,48.84a8,8,0,0,1,10.74-3.57l16,8a8,8,0,0,1-7.16,14.31l-16-8A8,8,0,0,1,16.84,92.42Zm192,15.16a8,8,0,0,1,3.58-10.73l16-8a8,8,0,1,1,7.16,14.31l-16,8a8,8,0,0,1-10.74-3.58Zm-48-55.16,8-16a8,8,0,0,1,14.32,7.16l-8,16a8,8,0,1,1-14.32-7.16Z"></path>
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
