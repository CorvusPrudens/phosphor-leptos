//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "communication"))]
#[component]
pub fn MatrixLogo(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM80,200H64a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H72V184h8a8,8,0,0,1,0,16Zm80-40a8,8,0,0,1-8-8V120a8,8,0,0,0-16,0v32a8,8,0,0,1-16,0V120a8,8,0,0,0-16,0v32a8,8,0,0,1-16,0V104a8,8,0,0,1,13.66-5.65A23.94,23.94,0,0,1,128,102.13,24,24,0,0,1,168,120v32A8,8,0,0,1,160,160Zm40,32a8,8,0,0,1-8,8H176a8,8,0,0,1,0-16h8V72h-8a8,8,0,0,1,0-16h16a8,8,0,0,1,8,8Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,40V216H40V40Z" opacity="0.2"></path>
    <path d="M72,216a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8H64a8,8,0,0,1,0,16H48V208H64A8,8,0,0,1,72,216ZM216,32H192a8,8,0,0,0,0,16h16V208H192a8,8,0,0,0,0,16h24a8,8,0,0,0,8-8V40A8,8,0,0,0,216,32Zm-32,88a32,32,0,0,0-56-21.13,31.93,31.93,0,0,0-40.71-6.15A8,8,0,0,0,72,96v64a8,8,0,0,0,16,0V120a16,16,0,0,1,32,0v40a8,8,0,0,0,16,0V120a16,16,0,0,1,32,0v40a8,8,0,0,0,16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M44,44V212H64a4,4,0,0,1,0,8H40a4,4,0,0,1-4-4V40a4,4,0,0,1,4-4H64a4,4,0,0,1,0,8Zm172-8H192a4,4,0,0,0,0,8h20V212H192a4,4,0,0,0,0,8h24a4,4,0,0,0,4-4V40A4,4,0,0,0,216,36ZM152,92a28,28,0,0,0-24,13.6,28,28,0,0,0-44-5.17V96a4,4,0,0,0-8,0v64a4,4,0,0,0,8,0V120a20,20,0,0,1,40,0v40a4,4,0,0,0,8,0V120a20,20,0,0,1,40,0v40a4,4,0,0,0,8,0V120A28,28,0,0,0,152,92Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M76,216a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V40A12,12,0,0,1,40,28H64a12,12,0,0,1,0,24H52V204H64A12,12,0,0,1,76,216ZM216,28H192a12,12,0,0,0,0,24h12V204H192a12,12,0,0,0,0,24h24a12,12,0,0,0,12-12V40A12,12,0,0,0,216,28ZM188,160V120a36,36,0,0,0-60-26.8,35.91,35.91,0,0,0-39.51-5.68A12,12,0,0,0,68,96v64a12,12,0,0,0,24,0V120a12,12,0,0,1,24,0v40a12,12,0,0,0,24,0V120a12,12,0,0,1,24,0v40a12,12,0,0,0,24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M46,46V210H64a6,6,0,0,1,0,12H40a6,6,0,0,1-6-6V40a6,6,0,0,1,6-6H64a6,6,0,0,1,0,12ZM216,34H192a6,6,0,0,0,0,12h18V210H192a6,6,0,0,0,0,12h24a6,6,0,0,0,6-6V40A6,6,0,0,0,216,34ZM152,90a30,30,0,0,0-24,12,30,30,0,0,0-42-6v0a6,6,0,0,0-12,0v64a6,6,0,0,0,12,0V120a18,18,0,0,1,36,0v40a6,6,0,0,0,12,0V120a18,18,0,0,1,36,0v40a6,6,0,0,0,12,0V120A30,30,0,0,0,152,90Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M72,216a8,8,0,0,1-8,8H40a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8H64a8,8,0,0,1,0,16H48V208H64A8,8,0,0,1,72,216ZM216,32H192a8,8,0,0,0,0,16h16V208H192a8,8,0,0,0,0,16h24a8,8,0,0,0,8-8V40A8,8,0,0,0,216,32Zm-32,88a32,32,0,0,0-56-21.13,31.93,31.93,0,0,0-40.71-6.15A8,8,0,0,0,72,96v64a8,8,0,0,0,16,0V120a16,16,0,0,1,32,0v40a8,8,0,0,0,16,0V120a16,16,0,0,1,32,0v40a8,8,0,0,0,16,0Z"></path>
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
