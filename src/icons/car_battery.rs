//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn CarBattery(
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
                <path d="M224,72H208V56a16,16,0,0,0-16-16H160a16,16,0,0,0-16,16V72H112V56A16,16,0,0,0,96,40H64A16,16,0,0,0,48,56V72H32A16,16,0,0,0,16,88v96a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V88A16,16,0,0,0,224,72ZM64,56H96V72H64Zm40,88H72a8,8,0,0,1,0-16h32a8,8,0,0,1,0,16Zm80,0h-8v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h8v-8a8,8,0,0,1,16,0v8h8a8,8,0,0,1,0,16Zm8-72H160V56h32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,88v96a8,8,0,0,1-8,8H32a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8H224A8,8,0,0,1,232,88Z"
        opacity="0.2"
    ></path>
    <path d="M192,136a8,8,0,0,1-8,8h-8v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h8v-8a8,8,0,0,1,16,0v8h8A8,8,0,0,1,192,136Zm-88-8H72a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16ZM240,88v96a16,16,0,0,1-16,16H32a16,16,0,0,1-16-16V88A16,16,0,0,1,32,72H48V56A16,16,0,0,1,64,40H96a16,16,0,0,1,16,16V72h32V56a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V72h16A16,16,0,0,1,240,88ZM160,72h32V56H160ZM64,72H96V56H64ZM224,184V88H32v96H224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M188,136a4,4,0,0,1-4,4H172v12a4,4,0,0,1-8,0V140H152a4,4,0,0,1,0-8h12V120a4,4,0,0,1,8,0v12h12A4,4,0,0,1,188,136Zm-84-4H72a4,4,0,0,0,0,8h32a4,4,0,0,0,0-8ZM236,88v96a12,12,0,0,1-12,12H32a12,12,0,0,1-12-12V88A12,12,0,0,1,32,76H52V56A12,12,0,0,1,64,44H96a12,12,0,0,1,12,12V76h40V56a12,12,0,0,1,12-12h32a12,12,0,0,1,12,12V76h20A12,12,0,0,1,236,88ZM156,76h40V56a4,4,0,0,0-4-4H160a4,4,0,0,0-4,4ZM60,76h40V56a4,4,0,0,0-4-4H64a4,4,0,0,0-4,4ZM228,88a4,4,0,0,0-4-4H32a4,4,0,0,0-4,4v96a4,4,0,0,0,4,4H224a4,4,0,0,0,4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M200,140a12,12,0,0,1-12,12h-4v4a12,12,0,0,1-24,0v-4h-4a12,12,0,0,1,0-24h4v-4a12,12,0,0,1,24,0v4h4A12,12,0,0,1,200,140ZM100,128H68a12,12,0,0,0,0,24h32a12,12,0,0,0,0-24ZM244,92v92a20,20,0,0,1-20,20H32a20,20,0,0,1-20-20V92A20,20,0,0,1,32,72H44V56A20,20,0,0,1,64,36H96a20,20,0,0,1,20,20V72h24V56a20,20,0,0,1,20-20h32a20,20,0,0,1,20,20V72h12A20,20,0,0,1,244,92ZM164,72h24V60H164ZM68,72H92V60H68ZM220,96H36v84H220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M190,136a6,6,0,0,1-6,6H174v10a6,6,0,0,1-12,0V142H152a6,6,0,0,1,0-12h10V120a6,6,0,0,1,12,0v10h10A6,6,0,0,1,190,136Zm-86-6H72a6,6,0,0,0,0,12h32a6,6,0,0,0,0-12ZM238,88v96a14,14,0,0,1-14,14H32a14,14,0,0,1-14-14V88A14,14,0,0,1,32,74H50V56A14,14,0,0,1,64,42H96a14,14,0,0,1,14,14V74h36V56a14,14,0,0,1,14-14h32a14,14,0,0,1,14,14V74h18A14,14,0,0,1,238,88ZM158,74h36V56a2,2,0,0,0-2-2H160a2,2,0,0,0-2,2ZM62,74H98V56a2,2,0,0,0-2-2H64a2,2,0,0,0-2,2ZM226,88a2,2,0,0,0-2-2H32a2,2,0,0,0-2,2v96a2,2,0,0,0,2,2H224a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,136a8,8,0,0,1-8,8h-8v8a8,8,0,0,1-16,0v-8h-8a8,8,0,0,1,0-16h8v-8a8,8,0,0,1,16,0v8h8A8,8,0,0,1,192,136Zm-88-8H72a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16ZM240,88v96a16,16,0,0,1-16,16H32a16,16,0,0,1-16-16V88A16,16,0,0,1,32,72H48V56A16,16,0,0,1,64,40H96a16,16,0,0,1,16,16V72h32V56a16,16,0,0,1,16-16h32a16,16,0,0,1,16,16V72h16A16,16,0,0,1,240,88ZM160,72h32V56H160ZM64,72H96V56H64ZM224,184V88H32v96H224Z"></path>
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
