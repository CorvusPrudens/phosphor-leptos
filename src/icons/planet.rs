//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn Planet(
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
                <path d="M245.11,60.68c-7.65-13.19-27.85-16.16-58.5-8.66A96,96,0,0,0,32.81,140.3C5.09,169,5.49,186,10.9,195.32,16,204.16,26.64,208,40.64,208a124.11,124.11,0,0,0,28.79-4,96,96,0,0,0,153.78-88.25c12.51-13,20.83-25.35,23.66-35.92C248.83,72.51,248.24,66.07,245.11,60.68Zm-13.69,15c-6.11,22.78-48.65,57.31-87.52,79.64-67.81,39-113.62,41.52-119.16,32-1.46-2.51-.65-7.24,2.22-13a80.06,80.06,0,0,1,10.28-15.05,95.53,95.53,0,0,0,6.23,14.18,4,4,0,0,0,4,2.12,122.14,122.14,0,0,0,16.95-3.32c21.23-5.55,46.63-16.48,71.52-30.78s47-30.66,62.45-46.15A122.74,122.74,0,0,0,209.7,82.45a4,4,0,0,0,.17-4.52,96.26,96.26,0,0,0-9.1-12.46c14.21-2.35,27.37-2.17,30.5,3.24C232.19,70.28,232.24,72.63,231.42,75.69Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M216,128a88,88,0,1,1-88-88A88,88,0,0,1,216,128Z" opacity="0.2"></path>
    <path d="M245.11,60.68c-7.65-13.19-27.84-16.16-58.5-8.66A95.93,95.93,0,0,0,32,128a98,98,0,0,0,.78,12.31C5.09,169,5.49,186,10.9,195.32,16,204.16,26.64,208,40.64,208a124.11,124.11,0,0,0,28.79-4A95.93,95.93,0,0,0,224,128a97.08,97.08,0,0,0-.77-12.25c12.5-13,20.82-25.35,23.65-35.92C248.83,72.51,248.24,66.07,245.11,60.68ZM128,48a80.11,80.11,0,0,1,78,62.2c-17.06,16.06-40.15,32.53-62.07,45.13C116.38,171.14,92.48,181,73.42,186.4A79.94,79.94,0,0,1,128,48ZM24.74,187.29c-1.46-2.51-.65-7.24,2.22-13a79.05,79.05,0,0,1,10.29-15.05,96,96,0,0,0,18,31.32C38,193.46,27.24,191.61,24.74,187.29ZM128,208a79.45,79.45,0,0,1-38.56-9.94,370,370,0,0,0,62.43-28.86c21.58-12.39,40.68-25.82,56.07-39.08A80.07,80.07,0,0,1,128,208ZM231.42,75.69c-1.7,6.31-6.19,13.53-12.63,21.13a95.69,95.69,0,0,0-18-31.35c14.21-2.35,27.37-2.17,30.5,3.24C232.19,70.28,232.24,72.63,231.42,75.69Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M241.66,62.68c-6.73-11.58-26.58-13.8-56-6.3A92,92,0,0,0,37,141.7c-21.38,21.69-29.44,40-22.68,51.62,4.13,7.11,13,10.71,26,10.71A115.25,115.25,0,0,0,65,200.9c1.74-.39,3.52-.82,5.32-1.28A92,92,0,0,0,220,128a92.84,92.84,0,0,0-1-13.73c12.77-13,21.31-25.28,24-35.47C244.69,72.54,244.23,67.12,241.66,62.68ZM128,44a84.13,84.13,0,0,1,82.38,67.56c-16.17,15.69-38.8,32.48-64.49,47.24-27.31,15.69-52.81,26.25-73.61,32A84,84,0,0,1,128,44ZM21.25,189.3C17,182,23.75,167.7,39,151.24A92,92,0,0,0,63,193.09C40.86,198,25.47,196.58,21.25,189.3ZM128,212a83.58,83.58,0,0,1-48-15.1c21.58-6.58,45.83-17.37,69.85-31.16,24.25-13.94,45.61-29.46,61.83-44.44.18,2.21.3,4.44.3,6.7A84.1,84.1,0,0,1,128,212ZM235.28,76.73c-2.13,8-8.58,17.65-18.25,28.06a92.07,92.07,0,0,0-24.12-41.92c21.85-4.82,37.59-3.46,41.83,3.83C236.2,69.23,236.39,72.6,235.28,76.73Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M248.59,58.67c-6.31-10.87-23-21.06-66.16-9.71A95.94,95.94,0,0,0,32,128q0,3.6.26,7.14C.56,166.86,1.1,186.4,7.44,197.33,13.4,207.61,25.3,212,40.68,212c9.79,0,21-1.78,32.95-4.91A95.94,95.94,0,0,0,224,128c0-2.41-.09-4.79-.27-7.16,14.31-14.38,23.86-28.21,27-40C253.55,70.42,251.12,63,248.59,58.67ZM128,56a72.11,72.11,0,0,1,70.19,56C184,124.73,165,138.59,141.92,151.86c-21.74,12.49-43.55,22.36-63.09,28.65A72,72,0,0,1,128,56ZM28.19,185.29c-.61-1.07-.17-8.22,10.67-21.71A95.77,95.77,0,0,0,52.35,187C35.12,189.61,28.85,186.41,28.19,185.29ZM128,200a71.66,71.66,0,0,1-22.56-3.64,394.1,394.1,0,0,0,48.42-23.69A388.11,388.11,0,0,0,198.43,143,72.12,72.12,0,0,1,128,200ZM227.57,74.65c-1.28,4.78-4.81,10.87-10.39,17.8A95.74,95.74,0,0,0,203.68,69c15.83-2.37,23.17,0,24.15,1.71C228,71,228.21,72.28,227.57,74.65Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M243.39,61.68c-7.24-12.48-27-15-57.24-7.49A93.92,93.92,0,0,0,34.05,128a94.5,94.5,0,0,0,.9,13c-21.86,22.38-29.56,40.78-22.29,53.32,4.5,7.76,14,11.69,27.86,11.69a116.38,116.38,0,0,0,25-3.16c1.45-.32,2.92-.68,4.41-1a93.95,93.95,0,0,0,151.19-86.89c12.65-13,21.11-25.32,23.86-35.6C246.76,72.53,246.24,66.59,243.39,61.68ZM128,46a82.12,82.12,0,0,1,80.19,64.94c-16,15.3-38.14,31.67-63.3,46.12C117.49,172.82,92.79,183,72.85,188.6A82,82,0,0,1,128,46ZM23,188.3c-3.52-6.07,2.31-18.56,15-33a94,94,0,0,0,21.07,36.62C39.42,195.74,26.39,194.08,23,188.3ZM128,210a81.41,81.41,0,0,1-43.35-12.45c20.68-6.71,43.56-17.06,66.22-30.08,22.83-13.12,43.13-27.67,59.05-41.91,0,.81.06,1.62.06,2.44A82.08,82.08,0,0,1,128,210ZM233.35,76.21c-1.88,7-7.28,15.49-15.36,24.61a93.92,93.92,0,0,0-21.1-36.7c15.82-3.05,32-3.49,36.12,3.58C234.2,69.75,234.31,72.62,233.35,76.21Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M245.11,60.68c-7.65-13.19-27.84-16.16-58.5-8.66A95.93,95.93,0,0,0,32,128a98,98,0,0,0,.78,12.31C5.09,169,5.49,186,10.9,195.32,16,204.16,26.64,208,40.64,208a124.11,124.11,0,0,0,28.79-4A95.93,95.93,0,0,0,224,128a97.08,97.08,0,0,0-.77-12.25c12.5-13,20.82-25.35,23.65-35.92C248.83,72.51,248.24,66.07,245.11,60.68ZM128,48a80.11,80.11,0,0,1,78,62.2c-17.06,16.06-40.15,32.53-62.07,45.13C116.38,171.14,92.48,181,73.42,186.4A79.94,79.94,0,0,1,128,48ZM24.74,187.29c-1.46-2.51-.65-7.24,2.22-13a79.05,79.05,0,0,1,10.29-15.05,96,96,0,0,0,18,31.32C38,193.46,27.24,191.61,24.74,187.29ZM128,208a79.45,79.45,0,0,1-38.56-9.94,370,370,0,0,0,62.43-28.86c21.58-12.39,40.68-25.82,56.07-39.08A80.07,80.07,0,0,1,128,208ZM231.42,75.69c-1.7,6.31-6.19,13.53-12.63,21.13a95.69,95.69,0,0,0-18-31.35c14.21-2.35,27.37-2.17,30.5,3.24C232.19,70.28,232.24,72.63,231.42,75.69Z"></path>
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
