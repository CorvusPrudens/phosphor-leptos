//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "design"))]
#[component]
pub fn Blueprint(
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
                <path d="M136,120h24v16H136ZM240,64V200a8,8,0,0,1-8,8H48a32,32,0,0,1-32-32V64A32,32,0,0,1,48,32H64a8,8,0,0,1,8,8V56H232A8,8,0,0,1,240,64ZM56,48H48A16,16,0,0,0,32,64v84.29A31.82,31.82,0,0,1,48,144h8Zm120,88V120h16a8,8,0,0,0,0-16H176V96a8,8,0,0,0-16,0v8H136V96a8,8,0,0,0-16,0v8H104a8,8,0,0,0,0,16h16v16H104a8,8,0,0,0,0,16h16v8a8,8,0,0,0,16,0v-8h24v8a8,8,0,0,0,16,0v-8h16a8,8,0,0,0,0-16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,64V200H48a24,24,0,0,1,0-48H64V64Z" opacity="0.2"></path>
    <path d="M232,56H72V40a8,8,0,0,0-8-8H48A32,32,0,0,0,16,64V176a32,32,0,0,0,32,32H232a8,8,0,0,0,8-8V64A8,8,0,0,0,232,56ZM32,64A16,16,0,0,1,48,48h8v96H48a31.82,31.82,0,0,0-16,4.29ZM224,192H48a16,16,0,0,1,0-32H64a8,8,0,0,0,8-8V72H224ZM104,136a8,8,0,0,0,0,16h16v8a8,8,0,0,0,16,0v-8h24v8a8,8,0,0,0,16,0v-8h16a8,8,0,0,0,0-16H176V120h16a8,8,0,0,0,0-16H176V96a8,8,0,0,0-16,0v8H136V96a8,8,0,0,0-16,0v8H104a8,8,0,0,0,0,16h16v16Zm32-16h24v16H136Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,60H68V40a4,4,0,0,0-4-4H48A28,28,0,0,0,20,64V176a28,28,0,0,0,28,28H232a4,4,0,0,0,4-4V64A4,4,0,0,0,232,60ZM28,64A20,20,0,0,1,48,44H60V148H48a27.94,27.94,0,0,0-20,8.42ZM228,196H48a20,20,0,0,1,0-40H64a4,4,0,0,0,4-4V68H228ZM104,140a4,4,0,0,0,0,8h20v12a4,4,0,0,0,8,0V148h32v12a4,4,0,0,0,8,0V148h20a4,4,0,0,0,0-8H172V116h20a4,4,0,0,0,0-8H172V96a4,4,0,0,0-8,0v12H132V96a4,4,0,0,0-8,0v12H104a4,4,0,0,0,0,8h20v24Zm28-24h32v24H132Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232,48H76V40A12,12,0,0,0,64,28H48A36,36,0,0,0,12,64V176a36,36,0,0,0,36,36H232a12,12,0,0,0,12-12V60A12,12,0,0,0,232,48ZM36,64A12,12,0,0,1,48,52h4v88H48a35.59,35.59,0,0,0-12,2.06ZM220,188H48a12,12,0,0,1,0-24H64a12,12,0,0,0,12-12V72H220ZM104,136a12,12,0,0,0,0,24h12v4a12,12,0,0,0,24,0v-4h16v4a12,12,0,0,0,24,0v-4h12a12,12,0,0,0,0-24H180V124h12a12,12,0,0,0,0-24H180V96a12,12,0,0,0-24,0v4H140V96a12,12,0,0,0-24,0v4H104a12,12,0,0,0,0,24h12v12Zm36-12h16v12H140Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,58H70V40a6,6,0,0,0-6-6H48A30,30,0,0,0,18,64V176a30,30,0,0,0,30,30H232a6,6,0,0,0,6-6V64A6,6,0,0,0,232,58ZM30,64A18,18,0,0,1,48,46H58V146H48a29.87,29.87,0,0,0-18,6ZM226,194H48a18,18,0,0,1,0-36H64a6,6,0,0,0,6-6V70H226ZM104,138a6,6,0,0,0,0,12h18v10a6,6,0,0,0,12,0V150h28v10a6,6,0,0,0,12,0V150h18a6,6,0,0,0,0-12H174V118h18a6,6,0,0,0,0-12H174V96a6,6,0,0,0-12,0v10H134V96a6,6,0,0,0-12,0v10H104a6,6,0,0,0,0,12h18v20Zm30-20h28v20H134Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,56H72V40a8,8,0,0,0-8-8H48A32,32,0,0,0,16,64V176a32,32,0,0,0,32,32H232a8,8,0,0,0,8-8V64A8,8,0,0,0,232,56ZM32,64A16,16,0,0,1,48,48h8v96H48a31.82,31.82,0,0,0-16,4.29ZM224,192H48a16,16,0,0,1,0-32H64a8,8,0,0,0,8-8V72H224ZM104,136a8,8,0,0,0,0,16h16v8a8,8,0,0,0,16,0v-8h24v8a8,8,0,0,0,16,0v-8h16a8,8,0,0,0,0-16H176V120h16a8,8,0,0,0,0-16H176V96a8,8,0,0,0-16,0v8H136V96a8,8,0,0,0-16,0v8H104a8,8,0,0,0,0,16h16v16Zm32-16h24v16H136Z"></path>
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
