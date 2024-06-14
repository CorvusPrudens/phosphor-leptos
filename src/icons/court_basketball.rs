//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "health", feature = "map"))]
#[component]
pub fn CourtBasketball(
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
                <path d="M240,92.23v71.54a4,4,0,0,1-4.41,4,40,40,0,0,1,0-79.52A4,4,0,0,1,240,92.23ZM20.41,167.76a40,40,0,0,0,0-79.52,4,4,0,0,0-4.41,4v71.54A4,4,0,0,0,20.41,167.76ZM116,48H32A16,16,0,0,0,16,64v4.13a4,4,0,0,0,3.8,4,56,56,0,0,1,0,111.74,4,4,0,0,0-3.8,4V192a16,16,0,0,0,16,16h84a4,4,0,0,0,4-4V52A4,4,0,0,0,116,48Zm108,0H140a4,4,0,0,0-4,4V204a4,4,0,0,0,4,4h84a16,16,0,0,0,16-16v-4.13a4,4,0,0,0-3.8-4,56,56,0,0,1,0-111.74,4,4,0,0,0,3.8-4V64A16,16,0,0,0,224,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,88v80H216a40,40,0,0,1,0-80ZM40,88H24v80H40a40,40,0,0,0,0-80Z"
        opacity="0.2"
    ></path>
    <path d="M224,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48Zm0,112h-8a32,32,0,0,1,0-64h8ZM32,96h8a32,32,0,0,1,0,64H32Zm0,80h8a48,48,0,0,0,0-96H32V64h88V192H32Zm192,16H136V64h88V80h-8a48,48,0,0,0,0,96h8v16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,52H32A12,12,0,0,0,20,64V192a12,12,0,0,0,12,12H224a12,12,0,0,0,12-12V64A12,12,0,0,0,224,52Zm4,112H216a36,36,0,0,1,0-72h12ZM28,92H40a36,36,0,0,1,0,72H28Zm0,100V172H40a44,44,0,0,0,0-88H28V64a4,4,0,0,1,4-4h92V196H32A4,4,0,0,1,28,192Zm196,4H132V60h92a4,4,0,0,1,4,4V84H216a44,44,0,0,0,0,88h12v20A4,4,0,0,1,224,196Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224,44H32A20,20,0,0,0,12,64V192a20,20,0,0,0,20,20H224a20,20,0,0,0,20-20V64A20,20,0,0,0,224,44Zm-4,112h-4a28,28,0,0,1,0-56h4ZM36,100h4a28,28,0,0,1,0,56H36Zm0,80h4A52,52,0,0,0,40,76H36V68h80V188H36Zm104,8V68h80v8h-4a52,52,0,0,0,0,104h4v8Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,50H32A14,14,0,0,0,18,64V192a14,14,0,0,0,14,14H224a14,14,0,0,0,14-14V64A14,14,0,0,0,224,50Zm2,112H216a34,34,0,0,1,0-68h10ZM30,94H40a34,34,0,0,1,0,68H30Zm0,98V174H40a46,46,0,0,0,0-92H30V64a2,2,0,0,1,2-2h90V194H32A2,2,0,0,1,30,192Zm194,2H134V62h90a2,2,0,0,1,2,2V82H216a46,46,0,0,0,0,92h10v18A2,2,0,0,1,224,194Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H224a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48Zm0,112h-8a32,32,0,0,1,0-64h8ZM32,96h8a32,32,0,0,1,0,64H32Zm0,80h8a48,48,0,0,0,0-96H32V64h88V192H32Zm192,16H136V64h88V80h-8a48,48,0,0,0,0,96h8v16Z"></path>
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
