//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn BluetoothSlash(
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
                <path d="M221.38,221.92a8,8,0,0,1-11.3-.54l-26.45-29.1L132.8,230.4a8,8,0,0,1-8.89.47,8.29,8.29,0,0,1-3.91-7.18V144L68.8,182.4a8,8,0,0,1-11.16-1.55,8.26,8.26,0,0,1,1.81-11.43l61.47-46.11L50.08,45.38A8,8,0,0,1,61.92,34.62l160,176A8,8,0,0,1,221.38,221.92ZM155,113.22a4,4,0,0,0,5.36.51L196.8,86.4a8,8,0,0,0,0-12.8l-64-48a8,8,0,0,0-10,.29A8.25,8.25,0,0,0,120,32.24V73.18a4,4,0,0,0,1,2.69Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M128,128l64,48-64,48Zm0-96v96l64-48Z" opacity="0.2"></path>
    <path d="M221.92,210.62l-160-176A8,8,0,0,0,50.08,45.38l70.84,77.93L59.2,169.6a8,8,0,1,0,9.6,12.8L120,144v80a8,8,0,0,0,12.8,6.4l50.83-38.12,26.45,29.1a8,8,0,1,0,11.84-10.76ZM136,208V144l11.73,8.8,25.08,27.59ZM120,71.63V32a8,8,0,0,1,12.8-6.4l64,48a8,8,0,0,1,0,12.8l-33.53,25.15a8,8,0,0,1-9.6-12.8l25-18.75L136,48V71.63a8,8,0,0,1-16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M219,213.31,59,37.31A4,4,0,0,0,53,42.69l74,81.42-.14,0a5.17,5.17,0,0,0-.51.18l-.15.06-.05,0a4,4,0,0,0-.56.35l-.05,0-64,48a4,4,0,0,0,4.8,6.4L124,136v88a4,4,0,0,0,2.21,3.58A4.05,4.05,0,0,0,128,228a4,4,0,0,0,2.4-.8l53.74-40.3L213,218.69a4,4,0,1,0,5.92-5.38ZM132,216V136l18.44,13.83L178.73,181ZM124,71.63V32a4,4,0,0,1,6.4-3.2l64,48a4,4,0,0,1,0,6.4l-33.53,25.15a4,4,0,0,1-2.4.8,4,4,0,0,1-2.4-7.2l29.26-22L132,40V71.63a4,4,0,0,1-8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224.88,207.93l-160-176A12,12,0,0,0,47.12,48.07L115,122.75,56.8,166.4a12,12,0,1,0,14.4,19.2L116,152v72a12,12,0,0,0,19.2,9.6l47.91-35.94,24,26.41a12,12,0,0,0,17.76-16.14ZM140,200V152l5,3.77,21.87,24.06ZM116,59.74V32a12,12,0,0,1,19.2-9.6l64,48a12,12,0,0,1,0,19.2l-27.1,20.33a12,12,0,0,1-14.4-19.2L172,80,140,56v3.74a12,12,0,0,1-24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M220.44,212,60.44,36A6,6,0,0,0,51.56,44l72.32,79.55L60.4,171.2a6,6,0,0,0,7.2,9.6L122,140v84a6,6,0,0,0,9.6,4.8l52.28-39.21L211.56,220a6,6,0,0,0,8.88-8.08ZM134,212V140l15.09,11.31,26.68,29.36ZM122,71.63V32a6,6,0,0,1,9.6-4.8l64,48a6,6,0,0,1,0,9.6L162.07,110a6,6,0,0,1-7.2-9.6L182,80,134,44V71.63a6,6,0,0,1-12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M221.92,210.62l-160-176A8,8,0,0,0,50.08,45.38l70.84,77.93L59.2,169.6a8,8,0,1,0,9.6,12.8L120,144v80a8,8,0,0,0,12.8,6.4l50.83-38.12,26.45,29.1a8,8,0,1,0,11.84-10.76ZM136,208V144l11.73,8.8,25.08,27.59ZM120,71.63V32a8,8,0,0,1,12.8-6.4l64,48a8,8,0,0,1,0,12.8l-33.53,25.15a8,8,0,0,1-9.6-12.8l25-18.75L136,48V71.63a8,8,0,0,1-16,0Z"></path>
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
