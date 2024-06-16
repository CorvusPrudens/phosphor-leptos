//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "health"))]
#[component]
pub fn Pill(
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
                <path d="M216.43,39.6a53.27,53.27,0,0,0-75.33,0L39.6,141.09a53.26,53.26,0,0,0,75.32,75.31L216.43,114.91A53.32,53.32,0,0,0,216.43,39.6Zm-11.32,64-50.75,50.74-52.69-52.68,50.75-50.75a37.26,37.26,0,0,1,52.69,52.69ZM189.68,82.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,1,1-11.31-11.32l24-24A8,8,0,0,1,189.68,82.34Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M160,160l-50.75,50.75a45.26,45.26,0,0,1-64,0h0a45.26,45.26,0,0,1,0-64L96,96Z"
        opacity="0.2"
    ></path>
    <path d="M216.43,39.6a53.27,53.27,0,0,0-75.33,0L39.6,141.09a53.26,53.26,0,0,0,75.32,75.31L216.43,114.91A53.32,53.32,0,0,0,216.43,39.6ZM103.61,205.09h0a37.26,37.26,0,0,1-52.7-52.69L96,107.31,148.7,160ZM205.11,103.6,160,148.69,107.32,96l45.1-45.09a37.26,37.26,0,0,1,52.69,52.69ZM189.68,82.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,1,1-11.31-11.32l24-24A8,8,0,0,1,189.68,82.34Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M186.84,85.17a4,4,0,0,1,0,5.66l-24,24a4,4,0,1,1-5.66-5.66l24-24A4,4,0,0,1,186.84,85.17Zm26.75,26.91L112.08,213.57a49.26,49.26,0,0,1-69.67-69.65L143.92,42.43a49.26,49.26,0,0,1,69.67,69.65ZM154.35,160,96,101.66,48.06,149.57a41.26,41.26,0,0,0,58.36,58.35ZM207.94,48.08a41.28,41.28,0,0,0-58.36,0L101.65,96,160,154.34l47.93-47.91A41.32,41.32,0,0,0,207.94,48.08Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M219.26,36.77a57.28,57.28,0,0,0-81,0L36.77,138.26a57.26,57.26,0,0,0,81,81L219.26,117.74A57.33,57.33,0,0,0,219.26,36.77ZM100.78,202.26a33.26,33.26,0,1,1-47-47L96,113l47,47Zm101.5-101.49L160,143,113,96l42.27-42.26a33.26,33.26,0,0,1,47,47Zm-9.77-25.26a12,12,0,0,1,0,17l-24,24a12,12,0,1,1-17-17l24-24A12,12,0,0,1,192.51,75.51Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M215,41a51.28,51.28,0,0,0-72.5,0L41,142.5A51.26,51.26,0,0,0,113.49,215L215,113.5A51.31,51.31,0,0,0,215,41ZM105,206.5A39.26,39.26,0,0,1,49.48,151L96,104.49,151.52,160ZM206.52,105,160,151.51,104.48,96,151,49.5A39.26,39.26,0,0,1,206.52,105ZM188.26,83.76a6,6,0,0,1,0,8.48l-24,24a6,6,0,0,1-8.49-8.48l24-24A6,6,0,0,1,188.26,83.76Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216.42,39.6a53.26,53.26,0,0,0-75.32,0L39.6,141.09a53.26,53.26,0,0,0,75.32,75.31h0L216.43,114.91A53.31,53.31,0,0,0,216.42,39.6ZM103.61,205.09h0a37.26,37.26,0,0,1-52.7-52.69L96,107.31,148.7,160ZM205.11,103.6,160,148.69,107.32,96l45.1-45.09a37.26,37.26,0,0,1,52.69,52.69ZM189.68,82.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,1,1-11.31-11.32l24-24A8,8,0,0,1,189.68,82.34Z"></path>
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
