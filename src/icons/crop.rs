//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Crop(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M244,192a12,12,0,0,1-12,12H204v28a12,12,0,0,1-24,0V204H64a12,12,0,0,1-12-12V76H24a12,12,0,0,1,0-24H52V24a12,12,0,0,1,24,0V180H232A12,12,0,0,1,244,192ZM104,76h76v76a12,12,0,0,0,24,0V64a12,12,0,0,0-12-12H104a12,12,0,0,0,0,24Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M192,64V192H64V64Z" opacity="0.2"/><path d="M240,192a8,8,0,0,1-8,8H200v32a8,8,0,0,1-16,0V200H64a8,8,0,0,1-8-8V72H24a8,8,0,0,1,0-16H56V24a8,8,0,0,1,16,0V184H232A8,8,0,0,1,240,192ZM96,72h88v88a8,8,0,0,0,16,0V64a8,8,0,0,0-8-8H96a8,8,0,0,0,0,16Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM120,80h48a8,8,0,0,1,8,8v48a8,8,0,0,1-16,0V96H120a8,8,0,0,1,0-16Zm72,96H176v16a8,8,0,0,1-16,0V176H88a8,8,0,0,1-8-8V96H64a8,8,0,0,1,0-16H80V64a8,8,0,0,1,16,0v96h96a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M238,192a6,6,0,0,1-6,6H198v34a6,6,0,0,1-12,0V198H64a6,6,0,0,1-6-6V70H24a6,6,0,0,1,0-12H58V24a6,6,0,0,1,12,0V186H232A6,6,0,0,1,238,192ZM96,70h90v90a6,6,0,0,0,12,0V64a6,6,0,0,0-6-6H96a6,6,0,0,0,0,12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,192a8,8,0,0,1-8,8H200v32a8,8,0,0,1-16,0V200H64a8,8,0,0,1-8-8V72H24a8,8,0,0,1,0-16H56V24a8,8,0,0,1,16,0V184H232A8,8,0,0,1,240,192ZM96,72h88v88a8,8,0,0,0,16,0V64a8,8,0,0,0-8-8H96a8,8,0,0,0,0,16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M236,192a4,4,0,0,1-4,4H196v36a4,4,0,0,1-8,0V196H64a4,4,0,0,1-4-4V68H24a4,4,0,0,1,0-8H60V24a4,4,0,0,1,8,0V188H232A4,4,0,0,1,236,192ZM96,68h92v92a4,4,0,0,0,8,0V64a4,4,0,0,0-4-4H96a4,4,0,0,0,0,8Z"/> }.into_view()
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
