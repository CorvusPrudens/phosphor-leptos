//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn Resize(
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
                <path d="M144,120v88a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V120a8,8,0,0,1,8-8h88A8,8,0,0,1,144,120Zm64,56a8,8,0,0,0-8,8v16H176a8,8,0,0,0,0,16h24a16,16,0,0,0,16-16V184A8,8,0,0,0,208,176Zm0-72a8,8,0,0,0-8,8v32a8,8,0,0,0,16,0V112A8,8,0,0,0,208,104Zm-8-64H184a8,8,0,0,0,0,16h16V72a8,8,0,0,0,16,0V56A16,16,0,0,0,200,40Zm-56,0H112a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16ZM48,88a8,8,0,0,0,8-8V56H72a8,8,0,0,0,0-16H56A16,16,0,0,0,40,56V80A8,8,0,0,0,48,88Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M136,120v88H48V120Z" opacity="0.2"></path>
    <path d="M136,112H48a8,8,0,0,0-8,8v88a8,8,0,0,0,8,8h88a8,8,0,0,0,8-8V120A8,8,0,0,0,136,112Zm-8,88H56V128h72Zm88-16v16a16,16,0,0,1-16,16H176a8,8,0,0,1,0-16h24V184a8,8,0,0,1,16,0Zm0-72v32a8,8,0,0,1-16,0V112a8,8,0,0,1,16,0Zm0-56V72a8,8,0,0,1-16,0V56H184a8,8,0,0,1,0-16h16A16,16,0,0,1,216,56Zm-64-8a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h32A8,8,0,0,1,152,48ZM40,80V56A16,16,0,0,1,56,40H72a8,8,0,0,1,0,16H56V80a8,8,0,0,1-16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M136,116H48a4,4,0,0,0-4,4v88a4,4,0,0,0,4,4h88a4,4,0,0,0,4-4V120A4,4,0,0,0,136,116Zm-4,88H52V124h80Zm80-20v16a12,12,0,0,1-12,12H176a4,4,0,0,1,0-8h24a4,4,0,0,0,4-4V184a4,4,0,0,1,8,0Zm0-72v32a4,4,0,0,1-8,0V112a4,4,0,0,1,8,0Zm0-56V72a4,4,0,0,1-8,0V56a4,4,0,0,0-4-4H184a4,4,0,0,1,0-8h16A12,12,0,0,1,212,56Zm-64-8a4,4,0,0,1-4,4H112a4,4,0,0,1,0-8h32A4,4,0,0,1,148,48ZM44,80V56A12,12,0,0,1,56,44H72a4,4,0,0,1,0,8H56a4,4,0,0,0-4,4V80a4,4,0,0,1-8,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M136,108H48a12,12,0,0,0-12,12v88a12,12,0,0,0,12,12h88a12,12,0,0,0,12-12V120A12,12,0,0,0,136,108Zm-12,88H60V132h64Zm96-8v12a20,20,0,0,1-20,20H180a12,12,0,0,1,0-24h16v-8a12,12,0,0,1,24,0Zm0-72v24a12,12,0,0,1-24,0V116a12,12,0,0,1,24,0Zm0-60V72a12,12,0,0,1-24,0V60H184a12,12,0,0,1,0-24h16A20,20,0,0,1,220,56Zm-68-8a12,12,0,0,1-12,12H116a12,12,0,0,1,0-24h24A12,12,0,0,1,152,48ZM36,76V56A20,20,0,0,1,56,36H68a12,12,0,0,1,0,24H60V76a12,12,0,0,1-24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M136,114H48a6,6,0,0,0-6,6v88a6,6,0,0,0,6,6h88a6,6,0,0,0,6-6V120A6,6,0,0,0,136,114Zm-6,88H54V126h76Zm84-18v16a14,14,0,0,1-14,14H176a6,6,0,0,1,0-12h24a2,2,0,0,0,2-2V184a6,6,0,0,1,12,0Zm0-72v32a6,6,0,0,1-12,0V112a6,6,0,0,1,12,0Zm0-56V72a6,6,0,0,1-12,0V56a2,2,0,0,0-2-2H184a6,6,0,0,1,0-12h16A14,14,0,0,1,214,56Zm-64-8a6,6,0,0,1-6,6H112a6,6,0,0,1,0-12h32A6,6,0,0,1,150,48ZM42,80V56A14,14,0,0,1,56,42H72a6,6,0,0,1,0,12H56a2,2,0,0,0-2,2V80a6,6,0,0,1-12,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,112H48a8,8,0,0,0-8,8v88a8,8,0,0,0,8,8h88a8,8,0,0,0,8-8V120A8,8,0,0,0,136,112Zm-8,88H56V128h72Zm88-16v16a16,16,0,0,1-16,16H176a8,8,0,0,1,0-16h24V184a8,8,0,0,1,16,0Zm0-72v32a8,8,0,0,1-16,0V112a8,8,0,0,1,16,0Zm0-56V72a8,8,0,0,1-16,0V56H184a8,8,0,0,1,0-16h16A16,16,0,0,1,216,56Zm-64-8a8,8,0,0,1-8,8H112a8,8,0,0,1,0-16h32A8,8,0,0,1,152,48ZM40,80V56A16,16,0,0,1,56,40H72a8,8,0,0,1,0,16H56V80a8,8,0,0,1-16,0Z"></path>
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
