//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance"))]
#[component]
pub fn CurrencyCny(
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
                <path d="M128,16A104,104,0,1,0,232,120,104.11,104.11,0,0,0,128,16ZM88,72h80a8,8,0,0,1,0,16H88a8,8,0,0,1,0-16Zm104,88a8,8,0,0,1-8,8H160a24,24,0,0,1-24-24V120H120a48.05,48.05,0,0,1-48,48,8,8,0,0,1,0-16,32,32,0,0,0,32-32H80a8,8,0,0,1,0-16h96a8,8,0,0,1,0,16H152v24a8,8,0,0,0,8,8h16v-8a8,8,0,0,1,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,56v56H64V56Z" opacity="0.2"></path>
    <path d="M56,56a8,8,0,0,1,8-8H192a8,8,0,0,1,0,16H64A8,8,0,0,1,56,56ZM216,160a8,8,0,0,0-8,8v16H176a16,16,0,0,1-16-16V120h48a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16H96v8a56.06,56.06,0,0,1-56,56,8,8,0,0,0,0,16,72.08,72.08,0,0,0,72-72v-8h32v48a32,32,0,0,0,32,32h40a8,8,0,0,0,8-8V168A8,8,0,0,0,216,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M60,56a4,4,0,0,1,4-4H192a4,4,0,0,1,0,8H64A4,4,0,0,1,60,56ZM216,164a4,4,0,0,0-4,4v20H176a20,20,0,0,1-20-20V116h52a4,4,0,0,0,0-8H48a4,4,0,0,0,0,8h52v12a60.07,60.07,0,0,1-60,60,4,4,0,0,0,0,8,68.07,68.07,0,0,0,68-68V116h40v52a28,28,0,0,0,28,28h40a4,4,0,0,0,4-4V168A4,4,0,0,0,216,164Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M52,56A12,12,0,0,1,64,44H192a12,12,0,0,1,0,24H64A12,12,0,0,1,52,56ZM216,156a12,12,0,0,0-12,12v12H176a12,12,0,0,1-12-12V124h44a12,12,0,0,0,0-24H48a12,12,0,0,0,0,24H92v4a52.06,52.06,0,0,1-52,52,12,12,0,0,0,0,24,76.08,76.08,0,0,0,76-76v-4h24v44a36,36,0,0,0,36,36h40a12,12,0,0,0,12-12V168A12,12,0,0,0,216,156Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M58,56a6,6,0,0,1,6-6H192a6,6,0,0,1,0,12H64A6,6,0,0,1,58,56ZM216,162a6,6,0,0,0-6,6v18H176a18,18,0,0,1-18-18V118h50a6,6,0,0,0,0-12H48a6,6,0,0,0,0,12H98v10a58.07,58.07,0,0,1-58,58,6,6,0,0,0,0,12,70.08,70.08,0,0,0,70-70V118h36v50a30,30,0,0,0,30,30h40a6,6,0,0,0,6-6V168A6,6,0,0,0,216,162Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M56,56a8,8,0,0,1,8-8H192a8,8,0,0,1,0,16H64A8,8,0,0,1,56,56ZM216,160a8,8,0,0,0-8,8v16H176a16,16,0,0,1-16-16V120h48a8,8,0,0,0,0-16H48a8,8,0,0,0,0,16H96v8a56.06,56.06,0,0,1-56,56,8,8,0,0,0,0,16,72.08,72.08,0,0,0,72-72v-8h32v48a32,32,0,0,0,32,32h40a8,8,0,0,0,8-8V168A8,8,0,0,0,216,160Z"></path>
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
