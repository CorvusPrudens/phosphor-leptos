//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects", feature = "commerce"))]
#[component]
pub fn Needle(
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
                <path d="M212.28,43.72a40,40,0,0,0-56.56,0l-24,24a8,8,0,0,0-2.23,4.3C120.69,123.28,36,208.73,34.36,210.33h0a8,8,0,0,0,11.31,11.32h0c.86-.87,86.83-86.31,138.32-95.15a8,8,0,0,0,4.3-2.23l24-24a40,40,0,0,0,0-56.56ZM189.66,77.66l-16,16a8,8,0,0,1-11.32-11.32l16-16a8,8,0,0,1,11.32,11.32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M206.63,94.63l-24,24C128,128,40,216,40,216s88-88,97.37-142.63l24-24a32,32,0,0,1,45.26,45.26Z"
        opacity="0.2"
    ></path>
    <path d="M189.66,66.34a8,8,0,0,1,0,11.32l-16,16a8,8,0,0,1-11.32-11.32l16-16A8,8,0,0,1,189.66,66.34ZM224,72a39.71,39.71,0,0,1-11.72,28.28l-24,24a8,8,0,0,1-4.3,2.23c-51.49,8.84-137.46,94.28-138.32,95.15h0a8,8,0,0,1-11.31-11.32h0C36,208.73,120.69,123.28,129.49,72a8,8,0,0,1,2.23-4.3l24-24A40,40,0,0,1,224,72Zm-16,0a24,24,0,0,0-41-17L144.77,77.29c-4.41,21.15-18.9,46.19-35.49,69.43,23.24-16.59,48.28-31.08,69.43-35.49L201,89A23.85,23.85,0,0,0,208,72Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M186.83,69.17a4,4,0,0,1,0,5.66l-16,16a4,4,0,0,1-5.66-5.66l16-16A4,4,0,0,1,186.83,69.17ZM220,72a35.76,35.76,0,0,1-10.54,25.46l-24,24a4.05,4.05,0,0,1-2.16,1.11c-21.65,3.72-52.74,21.46-89.91,51.33-28.34,22.77-50.34,44.71-50.56,44.93l-.24.22h0a4,4,0,0,1-5.42-5.88h0C37.39,213,59.33,191,82.1,162.61,112,125.44,129.71,94.35,133.43,72.7a4.05,4.05,0,0,1,1.11-2.16l24-24A36,36,0,0,1,220,72Zm-8,0a28,28,0,0,0-47.8-19.8L141.08,75.32c-5.33,28-29.48,63.42-52.51,92.11,28.69-23,64.16-47.18,92.11-52.52L203.8,91.8A27.81,27.81,0,0,0,212,72Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M215.11,40.89a44,44,0,0,0-62.22,0l-24,24a11.88,11.88,0,0,0-3.34,6.45C118.4,113,55.31,183.7,31.55,207.48l0,0a12,12,0,0,0,17,17h0c23.75-23.73,94.49-86.88,136.18-94a12,12,0,0,0,6.45-3.34l24-24a44,44,0,0,0,0-62.22Zm-17,45.25-21.42,21.43c-14.85,3.31-32.4,11.41-52.47,24.18,12.77-20.07,20.87-37.62,24.18-52.47l21.43-21.42a20,20,0,1,1,28.28,28.28Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M188.24,67.76a6,6,0,0,1,0,8.48l-16,16a6,6,0,0,1-8.48-8.48l16-16A6,6,0,0,1,188.24,67.76ZM222,72a37.74,37.74,0,0,1-11.13,26.87l-24,24a6,6,0,0,1-3.23,1.67c-52.14,9-138.53,94.84-139.4,95.7a5.81,5.81,0,0,1-1.82,1.25h0A6.12,6.12,0,0,1,40,222a6,6,0,0,1-4.24-10.24h0c1.4-1.41,86.78-87.44,95.69-139.39a6,6,0,0,1,1.67-3.23l24-24A38,38,0,0,1,222,72Zm-12,0a26,26,0,0,0-44.38-18.38L142.93,76.3c-4.14,20.79-18.62,47.61-43.13,79.9,32.29-24.51,59.11-39,79.9-43.13l22.68-22.69A25.79,25.79,0,0,0,210,72Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M189.66,66.34a8,8,0,0,1,0,11.32l-16,16a8,8,0,0,1-11.32-11.32l16-16A8,8,0,0,1,189.66,66.34ZM224,72a39.71,39.71,0,0,1-11.72,28.28l-24,24a8,8,0,0,1-4.3,2.23c-51.49,8.84-137.46,94.28-138.32,95.15h0a8,8,0,0,1-11.31-11.32h0C36,208.73,120.69,123.28,129.49,72a8,8,0,0,1,2.23-4.3l24-24A40,40,0,0,1,224,72Zm-16,0a24,24,0,0,0-41-17L144.77,77.29c-4.41,21.15-18.9,46.19-35.49,69.43,23.24-16.59,48.28-31.08,69.43-35.49L201,89A23.85,23.85,0,0,0,208,72Z"></path>
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
