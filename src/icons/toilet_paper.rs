//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn ToiletPaper(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M80,120a16,16,0,1,1-16-16A16,16,0,0,1,80,120Zm164,0v88a20,20,0,0,1-20,20H112a20,20,0,0,1-20-20v-9.61C84,207.07,74.46,212,64,212c-29.65,0-52-39.55-52-92S34.35,28,64,28H192C221.65,28,244,67.55,244,120ZM92,120c0-19.26-3.81-37.92-10.45-51.2C76.29,58.28,69.73,52,64,52S51.71,58.28,46.45,68.8C39.81,82.08,36,100.74,36,120s3.81,37.92,10.45,51.2C51.71,181.72,58.27,188,64,188s12.29-6.28,17.55-16.8C88.19,157.92,92,139.26,92,120Zm128,84V132h-8a12,12,0,0,1,0-24h7.5c-1.23-14.85-4.7-28.71-9.95-39.2S197.73,52,192,52H99.74c8.61,14.11,14.35,33.56,15.86,56H124a12,12,0,0,1,0,24h-8v72Zm-48-96h-8a12,12,0,0,0,0,24h8a12,12,0,0,0,0-24Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M104,120c0,44.18-17.91,80-40,80s-40-35.82-40-80S41.91,40,64,40,104,75.82,104,120Z" opacity="0.2"/><path d="M76,120a12,12,0,1,1-12-12A12,12,0,0,1,76,120Zm164,0v88a16,16,0,0,1-16,16H112a16,16,0,0,1-16-16V186.35C87.37,200.37,76.18,208,64,208c-13.87,0-26.46-9.89-35.44-27.85C20.46,164,16,142.59,16,120s4.46-43.95,12.56-60.15C37.54,41.89,50.13,32,64,32H192c13.87,0,26.46,9.89,35.44,27.85C235.54,76.05,240,97.41,240,120ZM96,120c0-42.43-16.86-72-32-72S32,77.57,32,120s16.86,72,32,72S96,162.43,96,120Zm128,88V128H208a8,8,0,0,1,0-16h15.79C221.84,73.9,206.16,48,192,48H92.12a73.6,73.6,0,0,1,7.32,11.85c7.14,14.28,11.44,32.56,12.37,52.15H128a8,8,0,0,1,0,16H112v80Zm-48-96H160a8,8,0,0,0,0,16h16a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M184,120a8,8,0,0,1-8,8H160a8,8,0,0,1,0-16h16A8,8,0,0,1,184,120Zm56,0v88a16,16,0,0,1-16,16H112a16,16,0,0,1-16-16V186.35C87.37,200.37,76.18,208,64,208c-13.87,0-26.46-9.89-35.44-27.85C20.46,164,16,142.59,16,120s4.46-43.95,12.56-60.15C37.54,41.89,50.13,32,64,32H192c13.87,0,26.46,9.89,35.44,27.85C235.54,76.05,240,97.41,240,120ZM76,120a12,12,0,1,0-12,12A12,12,0,0,0,76,120Zm148,8H208a8,8,0,0,1,0-16h15.79C221.84,73.9,206.16,48,192,48H92.12a73.6,73.6,0,0,1,7.32,11.85c7.14,14.28,11.44,32.56,12.37,52.15H128a8,8,0,0,1,0,16H112v80H224Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M74,120a10,10,0,1,1-10-10A10,10,0,0,1,74,120Zm164,0v88a14,14,0,0,1-14,14H112a14,14,0,0,1-14-14V178.48C89.65,195.49,77.6,206,64,206c-25.79,0-46-37.78-46-86S38.21,34,64,34H192C217.79,34,238,71.78,238,120ZM98,120c0-44.26-17.58-74-34-74S30,75.74,30,120s17.58,74,34,74S98,164.27,98,120Zm128,88V126H208a6,6,0,0,1,0-12h17.88C224.37,73.08,207.67,46,192,46H87.76c12.57,13.92,21.09,38.74,22.12,68H128a6,6,0,0,1,0,12H110v82a2,2,0,0,0,2,2H224A2,2,0,0,0,226,208Zm-50-94H160a6,6,0,0,0,0,12h16a6,6,0,0,0,0-12Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M76,120a12,12,0,1,1-12-12A12,12,0,0,1,76,120Zm164,0v88a16,16,0,0,1-16,16H112a16,16,0,0,1-16-16V186.35C87.37,200.37,76.18,208,64,208c-13.87,0-26.46-9.89-35.44-27.85C20.46,164,16,142.59,16,120s4.46-43.95,12.56-60.15C37.54,41.89,50.13,32,64,32H192c13.87,0,26.46,9.89,35.44,27.85C235.54,76.05,240,97.41,240,120ZM96,120c0-42.43-16.86-72-32-72S32,77.57,32,120s16.86,72,32,72S96,162.43,96,120Zm128,88V128H208a8,8,0,0,1,0-16h15.79C221.84,73.9,206.16,48,192,48H92.12a73.6,73.6,0,0,1,7.32,11.85c7.14,14.28,11.44,32.56,12.37,52.15H128a8,8,0,0,1,0,16H112v80Zm-48-96H160a8,8,0,0,0,0,16h16a8,8,0,0,0,0-16Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M72,120a8,8,0,1,1-8-8A8,8,0,0,1,72,120Zm164,0v88a12,12,0,0,1-12,12H112a12,12,0,0,1-12-12V168.86c-7.9,21.4-21,35.14-36,35.14-24.67,0-44-36.9-44-84S39.33,36,64,36H192C216.67,36,236,72.9,236,120Zm-136,0c0-41.2-16.49-76-36-76S28,78.8,28,120s16.49,76,36,76S100,161.2,100,120Zm128,88V124H208a4,4,0,0,1,0-8H228c-1-39.42-17.07-72-35.95-72H83.05c14.29,12.8,24.14,39.76,24.9,72h20a4,4,0,0,1,0,8H108v84a4,4,0,0,0,4,4H224A4,4,0,0,0,228,208Zm-52-92H160a4,4,0,0,0,0,8h16a4,4,0,0,0,0-8Z"/> }.into_view()
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
