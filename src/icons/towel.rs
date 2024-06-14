//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn Towel(
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
                <path d="M224,48V152a8,8,0,0,1-8.53,8,8.17,8.17,0,0,1-7.47-8.25V48a8,8,0,0,0-8.55-8A8.19,8.19,0,0,0,192,48.28V180a4,4,0,0,1-4,4H52a4,4,0,0,1-4-4V48A24,24,0,0,1,72,24H200A24,24,0,0,1,224,48ZM188,200H52a4,4,0,0,0-4,4v12a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V204A4,4,0,0,0,188,200Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M184,192v24a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V192Z" opacity="0.2"></path>
    <path d="M200,24H72A24,24,0,0,0,48,48V216a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V48a8,8,0,0,1,16,0V152a8,8,0,0,0,16,0V48A24,24,0,0,0,200,24ZM72,40H177.37A23.84,23.84,0,0,0,176,48V184H64V48A8,8,0,0,1,72,40ZM64,216V200H176v16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M200,28H72A20,20,0,0,0,52,48V216a12,12,0,0,0,12,12H176a12,12,0,0,0,12-12V48a12,12,0,0,1,24,0V152a4,4,0,0,0,8,0V48A20,20,0,0,0,200,28ZM72,36H184a19.86,19.86,0,0,0-4,12V188H60V48A12,12,0,0,1,72,36ZM176,220H64a4,4,0,0,1-4-4V196H180v20A4,4,0,0,1,176,220Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M197.16,20c-.47,0-.93,0-1.39,0H72A28,28,0,0,0,44,48V216a20,20,0,0,0,20,20H168a20,20,0,0,0,20-20V52A8,8,0,0,1,196,44h.35a8.33,8.33,0,0,1,7.7,8.48V148a12,12,0,0,0,24,0V52.48A32.13,32.13,0,0,0,197.16,20ZM72,44h93a32.24,32.24,0,0,0-1,8V172H68V48A4,4,0,0,1,72,44ZM68,212V196h96v16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200,26H72A22,22,0,0,0,50,48V216a14,14,0,0,0,14,14H176a14,14,0,0,0,14-14V48a10,10,0,0,1,20,0V152a6,6,0,0,0,12,0V48A22,22,0,0,0,200,26ZM72,38H180.41A21.84,21.84,0,0,0,178,48V186H62V48A10,10,0,0,1,72,38ZM176,218H64a2,2,0,0,1-2-2V198H178v18A2,2,0,0,1,176,218Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200,24H72A24,24,0,0,0,48,48V216a16,16,0,0,0,16,16H176a16,16,0,0,0,16-16V48a8,8,0,0,1,16,0V152a8,8,0,0,0,16,0V48A24,24,0,0,0,200,24ZM72,40H177.37A23.84,23.84,0,0,0,176,48V184H64V48A8,8,0,0,1,72,40ZM64,216V200H176v16Z"></path>
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
