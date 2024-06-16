//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn TextHTwo(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,160a8,8,0,0,1-16,0V128H72v32a8,8,0,0,1-16,0V80a8,8,0,0,1,16,0v32h40V80a8,8,0,0,1,16,0Zm64,24H152a8,8,0,0,1-6.4-12.8l36-48a12,12,0,1,0-19.15-14.46,13.06,13.06,0,0,0-2.58,4.81,8,8,0,1,1-15.68-3.18,28.17,28.17,0,1,1,50.2,22.44L168,168h24a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M240,72V208H56a16,16,0,0,1-16-16V56H224A16,16,0,0,1,240,72Z" opacity="0.2"></path>
    <path d="M248,208a8,8,0,0,1-8,8H192a8,8,0,0,1-6.4-12.8l43.16-57.56a16,16,0,1,0-25.54-19.27,16.28,16.28,0,0,0-2.32,4.3,8,8,0,1,1-15.08-5.34,32,32,0,1,1,55.73,29.93L208,200h32A8,8,0,0,1,248,208ZM144,48a8,8,0,0,0-8,8v52H48V56a8,8,0,0,0-16,0V176a8,8,0,0,0,16,0V124h88v52a8,8,0,0,0,16,0V56A8,8,0,0,0,144,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M148,56V176a4,4,0,0,1-8,0V120H44v56a4,4,0,0,1-8,0V56a4,4,0,0,1,8,0v56h96V56a4,4,0,0,1,8,0Zm92,148H200l38.36-51.15a28,28,0,1,0-48.77-26.18,4,4,0,1,0,7.54,2.66A20.36,20.36,0,0,1,200,124,20,20,0,0,1,232,148L188.8,205.6A4,4,0,0,0,192,212h48a4,4,0,0,0,0-8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M156,56V176a12,12,0,0,1-24,0V128H52v48a12,12,0,0,1-24,0V56a12,12,0,0,1,24,0v48h80V56a12,12,0,0,1,24,0Zm84,140H216l28.74-38.33A36,36,0,1,0,182.05,124a12,12,0,0,0,22.63,8,11.67,11.67,0,0,1,1.73-3.22,12,12,0,1,1,19.15,14.46L182.4,200.8A12,12,0,0,0,192,220h48a12,12,0,0,0,0-24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M150,56V176a6,6,0,0,1-12,0V122H46v54a6,6,0,0,1-12,0V56a6,6,0,0,1,12,0v54h92V56a6,6,0,0,1,12,0Zm90,146H204L240,154.05A30,30,0,1,0,187.71,126,6,6,0,1,0,199,130a18,18,0,0,1,14.47-11.82,18,18,0,0,1,16.87,28.66L187.2,204.4A6,6,0,0,0,192,214h48a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,56V176a8,8,0,0,1-16,0V124H48v52a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0v52h88V56a8,8,0,0,1,16,0Zm88,144H208l33.55-44.74a32,32,0,1,0-55.73-29.93,8,8,0,1,0,15.08,5.34,16.28,16.28,0,0,1,2.32-4.3,16,16,0,1,1,25.54,19.27L185.6,203.2A8,8,0,0,0,192,216h48a8,8,0,0,0,0-16Z"></path>
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
