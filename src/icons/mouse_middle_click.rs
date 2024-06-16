//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn MouseMiddleClick(
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
                <path d="M144,16H112A64.07,64.07,0,0,0,48,80v96a64.07,64.07,0,0,0,64,64h32a64.07,64.07,0,0,0,64-64V80A64.07,64.07,0,0,0,144,16Zm48,64v24H152V88a16,16,0,0,0-16-16V32h8A48.05,48.05,0,0,1,192,80ZM112,32h8V72a16,16,0,0,0-16,16v16H64V80A48.05,48.05,0,0,1,112,32Zm32,192H112a48.05,48.05,0,0,1-48-48V120h40v16a16,16,0,0,0,16,16h16a16,16,0,0,0,16-16V120h40v56A48.05,48.05,0,0,1,144,224Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M144,88v48a8,8,0,0,1-8,8H120a8,8,0,0,1-8-8V88a8,8,0,0,1,8-8h16A8,8,0,0,1,144,88Z"
        opacity="0.2"
    ></path>
    <path d="M144,16H112A64.07,64.07,0,0,0,48,80v96a64.07,64.07,0,0,0,64,64h32a64.07,64.07,0,0,0,64-64V80A64.07,64.07,0,0,0,144,16Zm48,64v24H152V88a16,16,0,0,0-16-16V32h8A48.05,48.05,0,0,1,192,80Zm-56,56H120V88h16v23.9a.51.51,0,0,0,0,.2ZM112,32h8V72a16,16,0,0,0-16,16v16H64V80A48.05,48.05,0,0,1,112,32Zm32,192H112a48.05,48.05,0,0,1-48-48V120h40v16a16,16,0,0,0,16,16h16a16,16,0,0,0,16-16V120h40v56A48.05,48.05,0,0,1,144,224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M144,20H112A60.07,60.07,0,0,0,52,80v96a60.07,60.07,0,0,0,60,60h32a60.07,60.07,0,0,0,60-60V80A60.07,60.07,0,0,0,144,20Zm52,60v28H148V88a12,12,0,0,0-12-12h-4V28h12A52.06,52.06,0,0,1,196,80Zm-80,8a4,4,0,0,1,4-4h16a4,4,0,0,1,4,4v48a4,4,0,0,1-4,4H120a4,4,0,0,1-4-4Zm-4-60h12V76h-4a12,12,0,0,0-12,12v20H60V80A52.06,52.06,0,0,1,112,28Zm32,200H112a52.06,52.06,0,0,1-52-52V116h48v20a12,12,0,0,0,12,12h16a12,12,0,0,0,12-12V116h48v60A52.06,52.06,0,0,1,144,228Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M144,12H112A68.07,68.07,0,0,0,44,80v96a68.07,68.07,0,0,0,68,68h32a68.07,68.07,0,0,0,68-68V80A68.07,68.07,0,0,0,144,12Zm44,68v20H160V88a20,20,0,0,0-20-20V36h4A44.05,44.05,0,0,1,188,80ZM120,92h16v40H120Zm-8-56h4V68A20,20,0,0,0,96,88v12H68V80A44.05,44.05,0,0,1,112,36Zm32,184H112a44.05,44.05,0,0,1-44-44V124H96v12a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20V124h28v52A44.05,44.05,0,0,1,144,220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M144,18H112A62.07,62.07,0,0,0,50,80v96a62.07,62.07,0,0,0,62,62h32a62.07,62.07,0,0,0,62-62V80A62.07,62.07,0,0,0,144,18Zm50,62v26H150V88a14,14,0,0,0-14-14h-2V30h10A50.06,50.06,0,0,1,194,80Zm-76,8a2,2,0,0,1,2-2h16a2,2,0,0,1,2,2v48a2,2,0,0,1-2,2H120a2,2,0,0,1-2-2Zm-6-58h10V74h-2a14,14,0,0,0-14,14v18H62V80A50.06,50.06,0,0,1,112,30Zm32,196H112a50.06,50.06,0,0,1-50-50V118h44v18a14,14,0,0,0,14,14h16a14,14,0,0,0,14-14V118h44v58A50.06,50.06,0,0,1,144,226Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M144,16H112A64.07,64.07,0,0,0,48,80v96a64.07,64.07,0,0,0,64,64h32a64.07,64.07,0,0,0,64-64V80A64.07,64.07,0,0,0,144,16Zm48,64v24H152V88a16,16,0,0,0-16-16V32h8A48.05,48.05,0,0,1,192,80Zm-56,56H120V88h16v23.9a.51.51,0,0,0,0,.2ZM112,32h8V72a16,16,0,0,0-16,16v16H64V80A48.05,48.05,0,0,1,112,32Zm32,192H112a48.05,48.05,0,0,1-48-48V120h40v16a16,16,0,0,0,16,16h16a16,16,0,0,0,16-16V120h40v56A48.05,48.05,0,0,1,144,224Z"></path>
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
