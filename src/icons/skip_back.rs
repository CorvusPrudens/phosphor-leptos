//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn SkipBack(
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
                <path d="M208,47.88V208.12a16,16,0,0,1-24.43,13.43L64,146.77V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0v69.23L183.57,34.45A15.95,15.95,0,0,1,208,47.88Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M200,47.88V208.12a8,8,0,0,1-12.19,6.65L59.7,134.65a7.83,7.83,0,0,1,0-13.3L187.81,41.23A8,8,0,0,1,200,47.88Z"
        opacity="0.2"
    ></path>
    <path d="M199.81,34a16,16,0,0,0-16.24.43L64,109.23V40a8,8,0,0,0-16,0V216a8,8,0,0,0,16,0V146.77l119.57,74.78A15.95,15.95,0,0,0,208,208.12V47.88A15.86,15.86,0,0,0,199.81,34ZM192,208,64.16,128,192,48.07Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M197.87,37.52a12,12,0,0,0-12.18.32L60,116.44V40a4,4,0,0,0-8,0V216a4,4,0,0,0,8,0V139.56l125.69,78.6a12,12,0,0,0,12.18.32A11.69,11.69,0,0,0,204,208.12V47.88A11.69,11.69,0,0,0,197.87,37.52ZM196,208.12a3.8,3.8,0,0,1-2,3.37,4,4,0,0,1-4.06-.11L61.82,131.26a3.83,3.83,0,0,1,0-6.52L189.94,44.62a4,4,0,0,1,2.11-.62,4,4,0,0,1,1.95.51,3.8,3.8,0,0,1,2,3.37Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M201.75,30.52a20,20,0,0,0-20.3.53L68,102V40a12,12,0,0,0-24,0V216a12,12,0,0,0,24,0V154l113.45,71A20,20,0,0,0,212,208.12V47.88A19.86,19.86,0,0,0,201.75,30.52ZM188,200.73,71.7,128,188,55.27Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M198.84,35.77a14,14,0,0,0-14.21.37L62,112.83V40a6,6,0,0,0-12,0V216a6,6,0,0,0,12,0V143.16l122.63,76.7a14,14,0,0,0,14.21.37A13.69,13.69,0,0,0,206,208.12V47.88A13.69,13.69,0,0,0,198.84,35.77ZM194,208.12a1.79,1.79,0,0,1-1,1.62,2,2,0,0,1-2-.05L62.88,129.56a1.82,1.82,0,0,1,0-3.12L191,46.31a2,2,0,0,1,1-.31,2.1,2.1,0,0,1,1,.26,1.79,1.79,0,0,1,1,1.62Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M199.81,34a16,16,0,0,0-16.24.43L64,109.23V40a8,8,0,0,0-16,0V216a8,8,0,0,0,16,0V146.77l119.57,74.78A15.95,15.95,0,0,0,208,208.12V47.88A15.86,15.86,0,0,0,199.81,34ZM192,208,64.16,128,192,48.07Z"></path>
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
