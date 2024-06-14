//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn NuclearPlant(
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
                <path d="M152,32h24a8,8,0,0,0,8-8,8,8,0,0,1,16,0,24,24,0,0,1-24,24H152a8,8,0,0,0-8,8,8,8,0,0,1-16,0A24,24,0,0,1,152,32ZM104,64a8,8,0,0,0,8-8,40,40,0,0,1,40-40h8a8,8,0,0,0,0-16h-8A56.06,56.06,0,0,0,96,56,8,8,0,0,0,104,64ZM248,216a8,8,0,0,1-8,8H16a8,8,0,0,1,0-16H32.74c13.77-27.83,29.48-68.69,31.12-112.66A15.91,15.91,0,0,1,79.85,80h88.33a16,16,0,0,1,16,15.28c2.1,47.84,23.84,92.37,35.29,112.72H240A8,8,0,0,1,248,216ZM168.18,96h-16c1.77,43.72,17.39,84.32,31.09,112h18C188.68,184.08,170.18,141.64,168.18,96Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M178.33,216H37.67C51.16,189.65,70,144.55,71.86,95.64a8,8,0,0,1,8-7.64h56.3a8,8,0,0,1,8,7.64C146,144.55,164.84,189.65,178.33,216Z"
        opacity="0.2"
    ></path>
    <path d="M240,208H219.45C208,187.65,186.26,143.12,184.16,95.28a16,16,0,0,0-16-15.28H79.85a15.91,15.91,0,0,0-16,15.34c-1.64,44-17.35,84.83-31.12,112.66H16a8,8,0,0,0,0,16H240a8,8,0,0,0,0-16ZM50.5,208c13.56-28.93,27.74-68.94,29.35-112l55.35-.06a7.46,7.46,0,0,0,1,.06c1.62,43.09,15.8,83.09,29.35,112Zm132.76,0c-13.7-27.69-29.32-68.29-31.09-112h16c2,45.66,20.5,88.1,33.06,112ZM152,32h24a8,8,0,0,0,8-8,8,8,0,0,1,16,0,24,24,0,0,1-24,24H152a8,8,0,0,0-8,8,8,8,0,0,1-16,0A24,24,0,0,1,152,32ZM96,56A56.06,56.06,0,0,1,152,0h8a8,8,0,0,1,0,16h-8a40,40,0,0,0-40,40,8,8,0,0,1-16,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M152,36h24a12,12,0,0,0,12-12,4,4,0,0,1,8,0,20,20,0,0,1-20,20H152a12,12,0,0,0-12,12,4,4,0,0,1-8,0A20,20,0,0,1,152,36ZM104,60a4,4,0,0,0,4-4,44.05,44.05,0,0,1,44-44h8a4,4,0,0,0,0-8h-8a52.06,52.06,0,0,0-52,52A4,4,0,0,0,104,60ZM244,216a4,4,0,0,1-4,4H16a4,4,0,0,1,0-8H35.21c14.21-28.21,30.94-70.65,32.65-116.51A11.93,11.93,0,0,1,79.85,84h88.33a12,12,0,0,1,12,11.46c2.21,50.34,25.71,97,37,116.54H240A4,4,0,0,1,244,216Zm-72.15-4c-14.14-28.88-30-70.85-31.7-116.21a4,4,0,0,0-4-3.79H79.85a4,4,0,0,0-4,3.79C74.16,141.15,58.29,183.12,44.15,212Zm36.09,0c-12.17-22-33.62-67.3-35.77-116.19a4,4,0,0,0-4-3.81H147.49a11.86,11.86,0,0,1,.65,3.49c1.71,45.86,18.44,88.3,32.65,116.51Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M92.83,50A59.79,59.79,0,0,1,152,0a12,12,0,0,1,0,24,35.85,35.85,0,0,0-35.5,30,12,12,0,0,1-11.82,10,11.33,11.33,0,0,1-2-.17A12,12,0,0,1,92.83,50ZM148,40a12,12,0,0,0,0,24h4a52.06,52.06,0,0,0,52-52,12,12,0,0,0-24,0,28,28,0,0,1-28,28ZM252,216a12,12,0,0,1-12,12H16a12,12,0,0,1,0-24H32.15c12.47-26.53,26.2-64.41,27.72-104.81A19.89,19.89,0,0,1,79.85,80h88.34a20,20,0,0,1,20,19.08c2,44.12,22.46,86.48,32.58,104.92H240A12,12,0,0,1,252,216ZM164.4,104h-16c2.36,38.59,15.49,74.53,27.47,100h17.78C181.92,180.52,167,143.66,164.4,104ZM58.48,204h91c-11.66-26.88-23.11-62.15-25.19-100H83.67C81.59,141.85,70.14,177.12,58.48,204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M152,34h24a10,10,0,0,0,10-10,6,6,0,0,1,12,0,22,22,0,0,1-22,22H152a10,10,0,0,0-10,10,6,6,0,0,1-12,0A22,22,0,0,1,152,34ZM104,62a6,6,0,0,0,6-6,42,42,0,0,1,42-42h8a6,6,0,0,0,0-12h-8A54.06,54.06,0,0,0,98,56,6,6,0,0,0,104,62ZM246,216a6,6,0,0,1-6,6H16a6,6,0,0,1,0-12H34c14-28,30.2-69.68,31.88-114.59A13.92,13.92,0,0,1,79.85,82h88.33a14,14,0,0,1,14,13.37C184.32,144.45,206.92,190,218.28,210H240A6,6,0,0,1,246,216ZM168.18,94H150c.06.46.1.94.12,1.41C151.82,140.32,168,182,182,210h22.55c-12.44-23.1-32.32-66.9-34.4-114.11A2,2,0,0,0,168.18,94ZM47.34,210H168.66c-13.86-28.94-28.86-69.92-30.51-114.14a2,2,0,0,0-2-1.86H79.85a2,2,0,0,0-2,1.86C76.2,140.08,61.2,181.06,47.34,210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,32h24a8,8,0,0,0,8-8,8,8,0,0,1,16,0,24,24,0,0,1-24,24H152a8,8,0,0,0-8,8,8,8,0,0,1-16,0A24,24,0,0,1,152,32ZM104,64a8,8,0,0,0,8-8,40,40,0,0,1,40-40h8a8,8,0,0,0,0-16h-8A56.06,56.06,0,0,0,96,56,8,8,0,0,0,104,64ZM248,216a8,8,0,0,1-8,8H16a8,8,0,0,1,0-16H32.74c13.77-27.83,29.48-68.69,31.12-112.66A15.91,15.91,0,0,1,79.85,80h88.33a16,16,0,0,1,16,15.28c2.1,47.84,23.84,92.37,35.29,112.72H240A8,8,0,0,1,248,216ZM168.18,96h-16c1.77,43.72,17.39,84.32,31.09,112h18C188.68,184.08,170.18,141.64,168.18,96ZM50.5,208h115C152,179.09,137.77,139.09,136.15,96a7.46,7.46,0,0,1-1-.06L79.85,96C78.24,139.06,64.06,179.07,50.5,208Z"></path>
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
