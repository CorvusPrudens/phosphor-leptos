//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "commerce", feature = "finance"))]
#[component]
pub fn StripeLogo(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,184c-22.06,0-40-14.35-40-32a8,8,0,0,1,16,0c0,8.67,11,16,24,16s24-7.33,24-16c0-9.48-8.61-13-26.88-18.26C109.37,129.2,89.78,123.55,89.78,104c0-18.24,16.43-32,38.22-32,15.72,0,29.18,7.3,35.12,19a8,8,0,1,1-14.27,7.22C145.64,91.94,137.65,88,128,88c-12.67,0-22.22,6.88-22.22,16,0,7,9,10.1,23.77,14.36C145.78,123,168,129.45,168,152,168,169.65,150.06,184,128,184Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,48V208a8,8,0,0,1-8,8H48a8,8,0,0,1-8-8V48a8,8,0,0,1,8-8H208A8,8,0,0,1,216,48Z"
        opacity="0.2"
    ></path>
    <path d="M168,152c0,17.65-17.94,32-40,32s-40-14.35-40-32a8,8,0,0,1,16,0c0,8.67,11,16,24,16s24-7.33,24-16c0-9.48-8.61-13-26.88-18.26C109.37,129.2,89.78,123.55,89.78,104c0-18.24,16.43-32,38.22-32,15.72,0,29.18,7.3,35.12,19a8,8,0,1,1-14.27,7.22C145.64,91.94,137.65,88,128,88c-12.67,0-22.22,6.88-22.22,16,0,7,9,10.1,23.77,14.36C145.78,123,168,129.45,168,152ZM224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48ZM208,208V48H48V208H208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,152c0,15.44-16.15,28-36,28s-36-12.56-36-28a4,4,0,0,1,8,0c0,11,12.56,20,28,20s28-9,28-20c0-12.58-11.79-16.92-29.78-22.11C111,125.51,93.78,120.54,93.78,104c0-16,14.71-28,34.22-28,14.2,0,26.3,6.46,31.55,16.85a4,4,0,1,1-7.13,3.61C148.53,88.78,139.17,84,128,84c-15,0-26.22,8.6-26.22,20,0,9.81,10.07,13.43,26.66,18.21S164,132.46,164,152ZM220,48V208a12,12,0,0,1-12,12H48a12,12,0,0,1-12-12V48A12,12,0,0,1,48,36H208A12,12,0,0,1,220,48Zm-8,0a4,4,0,0,0-4-4H48a4,4,0,0,0-4,4V208a4,4,0,0,0,4,4H208a4,4,0,0,0,4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,28H48A20,20,0,0,0,28,48V208a20,20,0,0,0,20,20H208a20,20,0,0,0,20-20V48A20,20,0,0,0,208,28Zm-4,176H52V52H204Zm-56-52c0-6.37-6.29-9.32-24-14.42-16.14-4.65-38.23-11-38.23-33.58,0-20.52,18.15-36,42.22-36,17.24,0,32.06,8.14,38.69,21.24a12,12,0,1,1-21.41,10.84C142.8,95.17,136,92,128,92c-10.22,0-18.22,5.27-18.22,12,0,1.09,0,2.21,3.28,4.17,4.18,2.48,11,4.45,17.6,6.35,8.75,2.52,17.8,5.13,25.38,9.86,13.19,8.23,16,19.56,16,27.62,0,20.19-19.33,36-44,36-21.26,0-39.46-12.35-43.27-29.38a12,12,0,0,1,23.42-5.24C109.3,158.49,117.21,164,128,164,137.44,164,148,158.87,148,152Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M166,152c0,16.54-17.05,30-38,30s-38-13.46-38-30a6,6,0,0,1,12,0c0,9.76,11.91,18,26,18s26-8.24,26-18c0-11.21-10.7-15.1-28.33-20.18-15.89-4.58-33.89-9.77-33.89-27.82,0-17.1,15.57-30,36.22-30,15,0,27.74,6.88,33.34,18a6,6,0,0,1-10.71,5.42C147.08,90.36,138.41,86,128,86c-13.81,0-24.22,7.74-24.22,18,0,8.41,9.52,11.76,25.21,16.29C145.48,125,166,131,166,152ZM222,48V208a14,14,0,0,1-14,14H48a14,14,0,0,1-14-14V48A14,14,0,0,1,48,34H208A14,14,0,0,1,222,48Zm-12,0a2,2,0,0,0-2-2H48a2,2,0,0,0-2,2V208a2,2,0,0,0,2,2H208a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,152c0,17.65-17.94,32-40,32s-40-14.35-40-32a8,8,0,0,1,16,0c0,8.67,11,16,24,16s24-7.33,24-16c0-9.48-8.61-13-26.88-18.26C109.37,129.2,89.78,123.55,89.78,104c0-18.24,16.43-32,38.22-32,15.72,0,29.18,7.3,35.12,19a8,8,0,1,1-14.27,7.22C145.64,91.94,137.65,88,128,88c-12.67,0-22.22,6.88-22.22,16,0,7,9,10.1,23.77,14.36C145.78,123,168,129.45,168,152ZM224,48V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48ZM208,208V48H48V208H208Z"></path>
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
