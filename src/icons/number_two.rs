//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance"))]
#[component]
pub fn NumberTwo(
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
                <path d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM160,176a8,8,0,0,1,0,16H96a8,8,0,0,1-5.79-13.52L145.9,120a24,24,0,0,0-35.73-32A23.33,23.33,0,0,0,107,92.38a8,8,0,0,1-14-7.77,40.22,40.22,0,0,1,5.28-7.38,40,40,0,0,1,59.45,53.54l-.16.16L114.66,176Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40Z"
        opacity="0.2"
    ></path>
    <path d="M176,208a8,8,0,0,1-8,8H88a8,8,0,0,1-6.4-12.8l71.94-95.92a32,32,0,1,0-51.1-38.53,32.5,32.5,0,0,0-3.78,6.46A8,8,0,1,1,84,68.8a48,48,0,1,1,82.33,48.09L104,200h64A8,8,0,0,1,176,208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M172,208a4,4,0,0,1-4,4H88a4,4,0,0,1-3.2-6.4l71.94-95.92A36,36,0,1,0,99.25,66.34,36.62,36.62,0,0,0,95,73.61a4,4,0,0,1-7.33-3.21,44.42,44.42,0,0,1,5.2-8.87,44,44,0,0,1,70.28,53L96,204h72A4,4,0,0,1,172,208Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M180,208a12,12,0,0,1-12,12H88a12,12,0,0,1-9.6-19.2l71.95-95.92a28,28,0,1,0-48-28.06,12,12,0,0,1-22-9.62,52.32,52.32,0,0,1,6.13-10.49,52,52,0,0,1,83.06,62.59L112,196h56A12,12,0,0,1,180,208Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M174,208a6,6,0,0,1-6,6H88a6,6,0,0,1-4.8-9.6l72-95.92a34,34,0,0,0-31.88-54.14A34.1,34.1,0,0,0,96.83,74.41a6,6,0,0,1-11-4.81,46.47,46.47,0,0,1,5.43-9.28,46,46,0,0,1,73.48,55.37L100,202h68A6,6,0,0,1,174,208Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M176,208a8,8,0,0,1-8,8H88a8,8,0,0,1-6.4-12.8l71.94-95.92a32,32,0,1,0-51.1-38.53,32.5,32.5,0,0,0-3.78,6.46A8,8,0,1,1,84,68.8a48,48,0,1,1,82.33,48.09L104,200h64A8,8,0,0,1,176,208Z"></path>
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
