//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map"))]
#[component]
pub fn GlobeX(
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
                <path d="M128,24a104,104,0,0,0,0,208,8,8,0,0,0,5-14.21l0,0c-1.12-.91-20.88-17.32-31.06-49.77h26a8,8,0,0,0,0-16H98.08a140.17,140.17,0,0,1,0-48h59.88A138,138,0,0,1,160,128a8,8,0,0,0,16,0,154.7,154.7,0,0,0-1.84-24h38.51A87.61,87.61,0,0,1,216,128a8,8,0,0,0,16,0A104.11,104.11,0,0,0,128,24ZM102,88a115.11,115.11,0,0,1,26-45,115.27,115.27,0,0,1,26,45Zm68.75,0a135.28,135.28,0,0,0-22.3-45.6,88.29,88.29,0,0,1,58,45.6Zm50.95,85.66L203.31,192l18.35,18.34a8,8,0,0,1-11.32,11.32L192,203.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L180.69,192l-18.35-18.34a8,8,0,0,1,11.32-11.32L192,180.69l18.34-18.35a8,8,0,0,1,11.32,11.32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M128,24a104,104,0,0,0,0,208,8,8,0,0,0,5-14.23h0c-1.12-.91-20.88-17.32-31.06-49.77h26a8,8,0,0,0,0-16H98.08a140.17,140.17,0,0,1,0-48h59.88A138,138,0,0,1,160,128a8,8,0,0,0,16,0,154.7,154.7,0,0,0-1.84-24h38.51A87.61,87.61,0,0,1,216,128a8,8,0,0,0,16,0A104.11,104.11,0,0,0,128,24ZM107.59,42.4A135.28,135.28,0,0,0,85.29,88H49.63A88.29,88.29,0,0,1,107.59,42.4Zm0,171.2a88.29,88.29,0,0,1-58-45.6H85.29A135.28,135.28,0,0,0,107.59,213.6ZM81.84,152H43.33a88.15,88.15,0,0,1,0-48H81.84a157.44,157.44,0,0,0,0,48ZM102,88a115.11,115.11,0,0,1,26-45,115.27,115.27,0,0,1,26,45Zm68.75,0a135.28,135.28,0,0,0-22.3-45.6,88.29,88.29,0,0,1,58,45.6Zm50.95,85.66L203.31,192l18.35,18.34a8,8,0,0,1-11.32,11.32L192,203.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L180.69,192l-18.35-18.34a8,8,0,0,1,11.32-11.32L192,180.69l18.34-18.35a8,8,0,0,1,11.32,11.32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28h0a100,100,0,0,0,0,200,4,4,0,0,0,2.49-7.11h0c-.3-.24-23.57-19.41-33.78-56.88H128a4,4,0,0,0,0-8H94.82a140.42,140.42,0,0,1,0-56h66.36A137.89,137.89,0,0,1,164,128a4,4,0,0,0,8,0,149.23,149.23,0,0,0-2.59-28h46.23A91.6,91.6,0,0,1,220,128a4,4,0,0,0,8,0A100.11,100.11,0,0,0,128,28Zm-10.46,8.6A129.39,129.39,0,0,0,88.35,92h-45A92.25,92.25,0,0,1,117.54,36.6Zm0,182.8A92.25,92.25,0,0,1,43.34,164h45A129.39,129.39,0,0,0,117.54,219.4ZM86.59,156H40.36a92.09,92.09,0,0,1,0-56H86.59a152.65,152.65,0,0,0,0,56ZM96.73,92C105,61.8,121.67,43.48,128,37.39c6.33,6.09,23,24.41,31.27,54.61Zm70.92,0a129.39,129.39,0,0,0-29.19-55.4A92.25,92.25,0,0,1,212.66,92Zm51.18,78.83L197.66,192l21.17,21.17a4,4,0,0,1-5.66,5.66L192,197.66l-21.17,21.17a4,4,0,0,1-5.66-5.66L186.34,192l-21.17-21.17a4,4,0,0,1,5.66-5.66L192,186.34l21.17-21.17a4,4,0,0,1,5.66,5.66Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20a108,108,0,0,0,0,216,12,12,0,0,0,7.49-21.37h0c-.25-.2-17.55-14.39-27.9-42.63H128a12,12,0,0,0,0-24H101.48a135.88,135.88,0,0,1,0-40h53A133.77,133.77,0,0,1,156,128a12,12,0,0,0,24,0,159,159,0,0,0-1.25-20h30.82A83.49,83.49,0,0,1,212,128a12,12,0,0,0,24,0A108.12,108.12,0,0,0,128,20ZM97.79,49.64A140.82,140.82,0,0,0,82.29,84H56.48A84.46,84.46,0,0,1,97.79,49.64Zm0,156.72A84.46,84.46,0,0,1,56.48,172H82.29A140.82,140.82,0,0,0,97.79,206.36ZM77.25,148H46.43a83.52,83.52,0,0,1,0-40H77.25a160.63,160.63,0,0,0,0,40Zm30.34-64a116.61,116.61,0,0,1,10-20.77A107.75,107.75,0,0,1,128,49a113.2,113.2,0,0,1,20.39,35Zm66.12,0a140.82,140.82,0,0,0-15.5-34.36A84.46,84.46,0,0,1,199.52,84Zm50.78,92.49L209,192l15.52,15.51a12,12,0,0,1-17,17L192,209l-15.51,15.52a12,12,0,0,1-17-17L175,192l-15.52-15.51a12,12,0,0,1,17-17L192,175l15.51-15.52a12,12,0,0,1,17,17Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26a102,102,0,0,0,0,204,6,6,0,0,0,3.76-10.67h0c-.1-.07-9.6-7.84-18.95-22.95A124,124,0,0,1,99.35,166H128a6,6,0,0,0,0-12H96.45a139.18,139.18,0,0,1,0-52h63.1A137.47,137.47,0,0,1,162,128a6,6,0,0,0,12,0,152.91,152.91,0,0,0-2.19-26h42.36A89.7,89.7,0,0,1,218,128a6,6,0,0,0,12,0A102.12,102.12,0,0,0,128,26ZM112.54,39.33A132.58,132.58,0,0,0,86.81,90H46.43A90.29,90.29,0,0,1,112.54,39.33Zm0,177.34A90.29,90.29,0,0,1,46.43,166H86.81A132.58,132.58,0,0,0,112.54,216.67ZM84.19,154H41.83a90.17,90.17,0,0,1,0-52H84.19a155.43,155.43,0,0,0,0,52ZM99.35,90a124,124,0,0,1,13.46-30.37A109.19,109.19,0,0,1,128,40.18a109.19,109.19,0,0,1,15.19,19.45A124,124,0,0,1,156.65,90Zm69.84,0a132.58,132.58,0,0,0-25.73-50.67A90.29,90.29,0,0,1,209.57,90Zm51.05,82.24L200.49,192l19.75,19.76a6,6,0,1,1-8.48,8.48L192,200.49l-19.76,19.75a6,6,0,0,1-8.48-8.48L183.51,192l-19.75-19.76a6,6,0,0,1,8.48-8.48L192,183.51l19.76-19.75a6,6,0,0,1,8.48,8.48Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24a104,104,0,0,0,0,208,8,8,0,0,0,5-14.23h0c-1.12-.91-20.88-17.32-31.06-49.77h26a8,8,0,0,0,0-16H98.08a140.17,140.17,0,0,1,0-48h59.88A138,138,0,0,1,160,128a8,8,0,0,0,16,0,154.7,154.7,0,0,0-1.84-24h38.51A87.61,87.61,0,0,1,216,128a8,8,0,0,0,16,0A104.11,104.11,0,0,0,128,24ZM107.59,42.4A135.28,135.28,0,0,0,85.29,88H49.63A88.29,88.29,0,0,1,107.59,42.4Zm0,171.2a88.29,88.29,0,0,1-58-45.6H85.29A135.28,135.28,0,0,0,107.59,213.6ZM81.84,152H43.33a88.15,88.15,0,0,1,0-48H81.84a157.44,157.44,0,0,0,0,48ZM102,88a115.11,115.11,0,0,1,26-45,115.27,115.27,0,0,1,26,45Zm68.75,0a135.28,135.28,0,0,0-22.3-45.6,88.29,88.29,0,0,1,58,45.6Zm50.95,85.66L203.31,192l18.35,18.34a8,8,0,0,1-11.32,11.32L192,203.31l-18.34,18.35a8,8,0,0,1-11.32-11.32L180.69,192l-18.35-18.34a8,8,0,0,1,11.32-11.32L192,180.69l18.34-18.35a8,8,0,0,1,11.32,11.32Z"></path>
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
