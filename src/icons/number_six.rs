//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn NumberSix(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M128,100a59.21,59.21,0,0,0-7.81.53l26.27-46.64a12,12,0,0,0-20.92-11.78L76,130.13A60,60,0,1,0,128,100Zm0,96a36,36,0,1,1,36-36A36,36,0,0,1,128,196Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M216,40V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V40A16,16,0,0,1,56,24H200A16,16,0,0,1,216,40Z" opacity="0.2"/><path d="M128,104a56,56,0,0,0-15.62,2.23L143,51.93A8,8,0,1,0,129,44.08l-49.55,88A56,56,0,1,0,128,104Zm0,96a40,40,0,1,1,40-40A40,40,0,0,1,128,200Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM128,192a44,44,0,0,1-38.3-65.62L123.38,68a8,8,0,0,1,13.86,8l-16.52,28.61A44.79,44.79,0,0,1,128,104a44,44,0,0,1,0,88Zm28-44a28,28,0,1,1-28-28A28,28,0,0,1,156,148Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,106a53.74,53.74,0,0,0-19.94,3.83L141.23,51a6,6,0,1,0-10.46-5.89l-49.54,88A54,54,0,1,0,128,106Zm0,96a42,42,0,1,1,42-42A42,42,0,0,1,128,202Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,104a56,56,0,0,0-15.62,2.23L143,51.93A8,8,0,1,0,129,44.08l-49.55,88A56,56,0,1,0,128,104Zm0,96a40,40,0,1,1,40-40A40,40,0,0,1,128,200Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,108a51.66,51.66,0,0,0-24.74,6.27L139.49,50a4,4,0,0,0-7-3.92L83,134.05A52,52,0,1,0,128,108Zm0,96a44,44,0,1,1,44-44A44.05,44.05,0,0,1,128,204Z"/> }.into_view()
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
