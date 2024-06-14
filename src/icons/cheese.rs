//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce"))]
#[component]
pub fn Cheese(
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
                <path d="M184,32a7.81,7.81,0,0,0-2.3.34l-160,48h0A8,8,0,0,0,16,88v16a8,8,0,0,0,8,8h7.46c13.45,0,24.79,11,24.54,24.46A24,24,0,0,1,32,160H24a8,8,0,0,0-8,8v24a8,8,0,0,0,8,8H224a16,16,0,0,0,16-16V88A56.06,56.06,0,0,0,184,32ZM80,184a32,32,0,0,1,64,0Zm88-48a32,32,0,0,1-31-40h62a32,32,0,0,1-31,40ZM78.51,80,185.12,48a40.06,40.06,0,0,1,38.07,32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M184,40,24,88v24h8a24,24,0,0,1,24,23.54C56.25,149,44.91,160,31.46,160H24v32H80v-8a32,32,0,0,1,64,0v8h80a8,8,0,0,0,8-8V88A48,48,0,0,0,184,40Zm-16,96a32,32,0,0,1-27.72-48h55.44A32,32,0,0,1,168,136Z"
        opacity="0.2"
    ></path>
    <path d="M184,32a7.81,7.81,0,0,0-2.3.34l-160,48h0A8,8,0,0,0,16,88v24a8,8,0,0,0,8,8h8a16.08,16.08,0,0,1,16,15.69A15.6,15.6,0,0,1,43.42,147a16.87,16.87,0,0,1-12,5.05H24a8,8,0,0,0-8,8v32a8,8,0,0,0,8,8H224a16,16,0,0,0,16-16V88A56.06,56.06,0,0,0,184,32Zm1.12,16a40.06,40.06,0,0,1,38.07,32H78.51ZM192,104a24,24,0,1,1-46.62-8h45.24A23.86,23.86,0,0,1,192,104ZM88,184a24,24,0,0,1,48,0Zm136,0H152a40,40,0,0,0-80,0H32V168a33,33,0,0,0,22.84-9.85A31.39,31.39,0,0,0,64,135.38,32.15,32.15,0,0,0,32,104V96h96.81a40,40,0,1,0,78.38,0H224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M184,36a4.05,4.05,0,0,0-1.15.17l-160,48v0A4,4,0,0,0,20,88v24a4,4,0,0,0,4,4h8a20.11,20.11,0,0,1,20,19.61,19.59,19.59,0,0,1-5.72,14.14A20.92,20.92,0,0,1,31.46,156H24a4,4,0,0,0-4,4v32a4,4,0,0,0,4,4H224a12,12,0,0,0,12-12V88A52.06,52.06,0,0,0,184,36Zm.57,8a44.06,44.06,0,0,1,43.24,40H51.25ZM196,104a28,28,0,1,1-53.29-12h50.58A28,28,0,0,1,196,104Zm-56,84H84v-4a28,28,0,0,1,56,0Zm88-4a4,4,0,0,1-4,4H148v-4a36,36,0,0,0-72,0v4H28V164h3.46A29,29,0,0,0,52,155.35a27.52,27.52,0,0,0,8-19.89A28.14,28.14,0,0,0,32,108H28V92H134.07a36,36,0,1,0,67.86,0H228Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M184,28a11.86,11.86,0,0,0-3.45.51l-160,48h0A12,12,0,0,0,12,88v24a12,12,0,0,0,12,12h8a12.07,12.07,0,0,1,12,11.76,11.6,11.6,0,0,1-3.43,8.38A12.88,12.88,0,0,1,31.46,148H24a12,12,0,0,0-12,12v32a12,12,0,0,0,12,12H224a20,20,0,0,0,20-20V88A60.07,60.07,0,0,0,184,28Zm1.64,24a36.06,36.06,0,0,1,32.3,24H105.76ZM152,100h32v4a16,16,0,0,1-32,0ZM96,180a16,16,0,0,1,32,0Zm124,0H152a40,40,0,0,0-80,0H36v-8.29A37.09,37.09,0,0,0,57.7,161,35.39,35.39,0,0,0,68,135.31a36.21,36.21,0,0,0-32-35.09V100h92v4a40,40,0,0,0,80,0v-4h12Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M184,34a5.92,5.92,0,0,0-1.72.25l-160,48h0A6,6,0,0,0,18,88v24a6,6,0,0,0,6,6h8a18.09,18.09,0,0,1,18,17.65,17.59,17.59,0,0,1-5.15,12.7A18.91,18.91,0,0,1,31.46,154H24a6,6,0,0,0-6,6v32a6,6,0,0,0,6,6H224a14,14,0,0,0,14-14V88A54.06,54.06,0,0,0,184,34Zm.85,12a42.07,42.07,0,0,1,40.72,36H64.88ZM194,104a26,26,0,1,1-50-10h48A25.87,25.87,0,0,1,194,104Zm-56,82H86v-2a26,26,0,0,1,52,0Zm88-2a2,2,0,0,1-2,2H150v-2a38,38,0,0,0-76,0v2H30V166h1.46a31,31,0,0,0,22-9.25A29.45,29.45,0,0,0,62,135.42,30.14,30.14,0,0,0,32,106H30V94H131.34a38,38,0,1,0,73.32,0H226Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M184,32a7.81,7.81,0,0,0-2.3.34l-160,48h0A8,8,0,0,0,16,88v24a8,8,0,0,0,8,8h8a16.08,16.08,0,0,1,16,15.69A15.6,15.6,0,0,1,43.42,147a16.87,16.87,0,0,1-12,5.05H24a8,8,0,0,0-8,8v32a8,8,0,0,0,8,8H224a16,16,0,0,0,16-16V88A56.06,56.06,0,0,0,184,32Zm1.12,16a40.06,40.06,0,0,1,38.07,32H78.51ZM192,104a24,24,0,1,1-46.62-8h45.24A23.86,23.86,0,0,1,192,104ZM88,184a24,24,0,0,1,48,0Zm136,0H152a40,40,0,0,0-80,0H32V168a33,33,0,0,0,22.84-9.85A31.39,31.39,0,0,0,64,135.38,32.15,32.15,0,0,0,32,104V96h96.81a40,40,0,1,0,78.38,0H224Z"></path>
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
