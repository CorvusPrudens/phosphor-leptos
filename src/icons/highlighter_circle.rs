//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor", feature = "office"))]
#[component]
pub fn HighlighterCircle(
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
                <path d="M201.54,54.46A104,104,0,0,0,54.46,201.54,104,104,0,0,0,201.54,54.46ZM96,210V152h64v58a88.33,88.33,0,0,1-64,0Zm94.22-19.78A88.09,88.09,0,0,1,176,201.77V152a16,16,0,0,0-16-16V72a8,8,0,0,0-11.58-7.16l-48,24A8,8,0,0,0,96,96v40a16,16,0,0,0-16,16v49.77a88,88,0,1,1,110.22-11.55Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M195.88,195.88A95.7,95.7,0,0,1,168,215.29V152a8,8,0,0,0-8-8h-8V72L104,96v48H96a8,8,0,0,0-8,8v63.29a96,96,0,1,1,107.88-19.41Z"
        opacity="0.2"
    ></path>
    <path d="M201.54,54.46A104,104,0,0,0,54.46,201.54,104,104,0,0,0,201.54,54.46ZM96,210V152h64v58a88.33,88.33,0,0,1-64,0Zm48-74H112V100.94l32-16Zm46.22,54.22A88.09,88.09,0,0,1,176,201.77V152a16,16,0,0,0-16-16V72a8,8,0,0,0-11.58-7.16l-48,24A8,8,0,0,0,96,96v40a16,16,0,0,0-16,16v49.77a88,88,0,1,1,110.22-11.55Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M198.71,57.29A100,100,0,1,0,57.29,198.71,100,100,0,1,0,198.71,57.29ZM92,212.7V152a4,4,0,0,1,4-4h64a4,4,0,0,1,4,4v60.7a92.42,92.42,0,0,1-72,0ZM148,140H108V98.47l40-20Zm45.05,53.05A92,92,0,0,1,172,208.83V152a12,12,0,0,0-12-12h-4V72a4,4,0,0,0-5.79-3.58l-48,24A4,4,0,0,0,100,96v44H96a12,12,0,0,0-12,12v56.83a92,92,0,1,1,109.05-15.78Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M204.37,51.59A108,108,0,1,0,236,128,108.11,108.11,0,0,0,204.37,51.59ZM140,136H116V111.39l24-12Zm-40,71.25V160h56v47.25a84.73,84.73,0,0,1-56,0Zm87.4-19.84a84.53,84.53,0,0,1-7.4,6.55V156a20,20,0,0,0-16-19.6V80a12,12,0,0,0-17.37-10.73l-48,24A12,12,0,0,0,92,104v32.41A20,20,0,0,0,76,156v38a84.53,84.53,0,0,1-7.4-6.55,84,84,0,1,1,118.8,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200.12,55.87A102,102,0,1,0,55.88,200.12,102,102,0,1,0,200.12,55.87ZM94,211.37V152a2,2,0,0,1,2-2h64a2,2,0,0,1,2,2v59.37a90.49,90.49,0,0,1-68,0ZM146,138H110V99.71l36-18Zm45.64,53.64h0A90.93,90.93,0,0,1,174,205.39V152a14,14,0,0,0-14-14h-2V72a6,6,0,0,0-8.68-5.37l-48,24A6,6,0,0,0,98,96v42H96a14,14,0,0,0-14,14v53.39a90.93,90.93,0,0,1-17.64-13.75,90,90,0,1,1,127.28,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M201.54,54.46A104,104,0,0,0,54.46,201.54,104,104,0,0,0,201.54,54.46ZM96,210V152h64v58a88.33,88.33,0,0,1-64,0Zm48-74H112V100.94l32-16Zm46.22,54.22A88.09,88.09,0,0,1,176,201.77V152a16,16,0,0,0-16-16V72a8,8,0,0,0-11.58-7.16l-48,24A8,8,0,0,0,96,96v40a16,16,0,0,0-16,16v49.77a88,88,0,1,1,110.22-11.55Z"></path>
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
