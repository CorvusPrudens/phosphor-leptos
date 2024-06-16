//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media"))]
#[component]
pub fn EjectSimple(
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
                <path d="M232,200a8,8,0,0,1-8,8H32a8,8,0,1,1,0-16H224A8,8,0,0,1,232,200ZM40.09,160H215.91a16.1,16.1,0,0,0,12.48-26.23L146.74,32.94a24.11,24.11,0,0,0-37.48,0L27.61,133.77A16.1,16.1,0,0,0,40.09,160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M215.92,152H40.08a8.1,8.1,0,0,1-6.26-13.2L115.48,38a16.1,16.1,0,0,1,25,0L222.18,138.8A8.1,8.1,0,0,1,215.92,152Z"
        opacity="0.2"
    ></path>
    <path d="M232,200a8,8,0,0,1-8,8H32a8,8,0,1,1,0-16H224A8,8,0,0,1,232,200ZM25.59,150.84a16,16,0,0,1,2-17.07L109.26,32.94a24.11,24.11,0,0,1,37.48,0l81.65,100.83A16.1,16.1,0,0,1,215.91,160H40.09A16,16,0,0,1,25.59,150.84ZM40,143.91s0,.09.08.11l175.83,0s.08-.09.08-.13L134.3,43a8.1,8.1,0,0,0-12.6,0L40,143.84A.28.28,0,0,0,40,143.91Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228,200a4,4,0,0,1-4,4H32a4,4,0,0,1,0-8H224A4,4,0,0,1,228,200ZM29.2,149.12a12,12,0,0,1,1.51-12.83L112.37,35.46a20.1,20.1,0,0,1,31.26,0l81.66,100.83A12.1,12.1,0,0,1,215.92,156H40.08A12,12,0,0,1,29.2,149.12Zm7.22-3.44A4,4,0,0,0,40.08,148H215.92a4,4,0,0,0,3.66-2.32,4,4,0,0,0-.51-4.36L137.41,40.5a12.09,12.09,0,0,0-18.82,0L36.93,141.32A4,4,0,0,0,36.42,145.68Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M236,200a12,12,0,0,1-12,12H32a12,12,0,1,1,0-24H224A12,12,0,0,1,236,200ZM22,152.57a20,20,0,0,1,2.52-21.32L106.16,30.43a28.08,28.08,0,0,1,43.68,0l81.65,100.82A20.1,20.1,0,0,1,215.91,164H40.09A19.9,19.9,0,0,1,22,152.57ZM48.3,140H207.7L131.2,45.53a4.11,4.11,0,0,0-6.4,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M230,200a6,6,0,0,1-6,6H32a6,6,0,0,1,0-12H224A6,6,0,0,1,230,200ZM27.39,150A14,14,0,0,1,29.16,135L110.82,34.2a22.1,22.1,0,0,1,34.36,0L226.84,135a14.09,14.09,0,0,1-10.93,23H40.09A14,14,0,0,1,27.39,150Zm10.83-5.16A2,2,0,0,0,40.09,146H215.91a2,2,0,0,0,1.87-1.18,2,2,0,0,0-.27-2.24L135.86,41.76a10.1,10.1,0,0,0-15.72,0L38.49,142.58A2,2,0,0,0,38.22,144.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,200a8,8,0,0,1-8,8H32a8,8,0,1,1,0-16H224A8,8,0,0,1,232,200ZM25.59,150.84a16,16,0,0,1,2-17.07L109.26,32.94a24.11,24.11,0,0,1,37.48,0l81.65,100.83A16.1,16.1,0,0,1,215.91,160H40.09A16,16,0,0,1,25.59,150.84ZM40,143.91s0,.09.08.11l175.83,0s.08-.09.08-.13L134.3,43a8.1,8.1,0,0,0-12.6,0L40,143.84A.28.28,0,0,0,40,143.91Z"></path>
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
