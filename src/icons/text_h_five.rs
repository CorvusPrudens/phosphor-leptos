//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn TextHFive(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M252,180a40,40,0,0,1-40,40,39.53,39.53,0,0,1-28.57-11.6,12,12,0,1,1,17.14-16.8A15.54,15.54,0,0,0,212,196a16,16,0,0,0,0-32,15.54,15.54,0,0,0-11.43,4.4A12,12,0,0,1,180.16,158l8-48A12,12,0,0,1,200,100h40a12,12,0,0,1,0,24H210.17l-2.71,16.23A45.39,45.39,0,0,1,212,140,40,40,0,0,1,252,180ZM144,44a12,12,0,0,0-12,12v48H52V56a12,12,0,0,0-24,0V176a12,12,0,0,0,24,0V128h80v48a12,12,0,0,0,24,0V56A12,12,0,0,0,144,44Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,64V200a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V56H232A8,8,0,0,1,240,64Z" opacity="0.2"/><path d="M152,56V176a8,8,0,0,1-16,0V124H48v52a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0v52h88V56a8,8,0,0,1,16,0Zm60,88a38.8,38.8,0,0,0-9.41,1.14L206.78,120H240a8,8,0,0,0,0-16H200a8,8,0,0,0-7.89,6.68l-8,48a8,8,0,0,0,13.6,6.92A19.73,19.73,0,0,1,212,160a20,20,0,0,1,0,40,19.73,19.73,0,0,1-14.29-5.6,8,8,0,1,0-11.42,11.2A35.54,35.54,0,0,0,212,216a36,36,0,0,0,0-72Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,160a8,8,0,0,1-16,0V128H72v32a8,8,0,0,1-16,0V80a8,8,0,0,1,16,0v32h40V80a8,8,0,0,1,16,0Zm40-40a32,32,0,1,1-21.34,55.85,8,8,0,0,1,10.67-11.92,16,16,0,1,0,0-24,8,8,0,0,1-13.17-7.61l8-38A8,8,0,0,1,160,88h32a8,8,0,0,1,0,16H166.49L163,120.37A34.08,34.08,0,0,1,168,120Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M246,180a34,34,0,0,1-34,34,33.6,33.6,0,0,1-24.29-9.8,6,6,0,0,1,8.58-8.4A21.65,21.65,0,0,0,212,202a22,22,0,0,0,0-44,21.65,21.65,0,0,0-15.71,6.2A6,6,0,0,1,186.08,159l8-48a6,6,0,0,1,5.92-5h40a6,6,0,0,1,0,12H205.08l-5,30A36,36,0,0,1,212,146,34,34,0,0,1,246,180ZM144,50a6,6,0,0,0-6,6v54H46V56a6,6,0,0,0-12,0V176a6,6,0,0,0,12,0V122h92v54a6,6,0,0,0,12,0V56A6,6,0,0,0,144,50Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M152,56V176a8,8,0,0,1-16,0V124H48v52a8,8,0,0,1-16,0V56a8,8,0,0,1,16,0v52h88V56a8,8,0,0,1,16,0Zm60,88a38.8,38.8,0,0,0-9.41,1.14L206.78,120H240a8,8,0,0,0,0-16H200a8,8,0,0,0-7.89,6.68l-8,48a8,8,0,0,0,13.6,6.92A19.73,19.73,0,0,1,212,160a20,20,0,0,1,0,40,19.73,19.73,0,0,1-14.29-5.6,8,8,0,1,0-11.42,11.2A35.54,35.54,0,0,0,212,216a36,36,0,0,0,0-72Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M244,180a32,32,0,0,1-32,32,31.62,31.62,0,0,1-22.86-9.2,4,4,0,0,1,5.72-5.6A23.67,23.67,0,0,0,212,204a24,24,0,0,0,0-48,23.67,23.67,0,0,0-17.14,6.8,4,4,0,0,1-6.81-3.46l8-48A4,4,0,0,1,200,108h40a4,4,0,0,1,0,8H203.39l-5.87,35.21A33.26,33.26,0,0,1,212,148,32,32,0,0,1,244,180ZM144,52a4,4,0,0,0-4,4v56H44V56a4,4,0,0,0-8,0V176a4,4,0,0,0,8,0V120h96v56a4,4,0,0,0,8,0V56A4,4,0,0,0,144,52Z"/> }.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
