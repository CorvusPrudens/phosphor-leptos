//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn AirplaneInFlight(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M228,216a12,12,0,0,1-12,12H72a12,12,0,0,1,0-24H216A12,12,0,0,1,228,216Zm24-80v24a12,12,0,0,1-12,12H61.07a43.72,43.72,0,0,1-42.14-31.36L4.86,93.75A20,20,0,0,1,24,68h8a12,12,0,0,1,8.48,3.51L61,92H76.27L69,70.32A20,20,0,0,1,88,44h8a12,12,0,0,1,8.48,3.51L149,92h59A44.05,44.05,0,0,1,252,136Zm-24,0a20,20,0,0,0-20-20H144a12,12,0,0,1-8.48-3.51L94.83,71.79l9.47,28.42A12,12,0,0,1,92.91,116H56a12,12,0,0,1-8.49-3.51L30.4,95.36l11.51,38.39A19.89,19.89,0,0,0,61.07,148H228Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M240,136v24H61.06a32,32,0,0,1-30.65-22.8L16.34,90.3A8,8,0,0,1,24,80h8l24,24H92.91L80.42,66.53A8,8,0,0,1,88,56h8l48,48h64A32,32,0,0,1,240,136Z" opacity="0.2"/><path d="M224,216a8,8,0,0,1-8,8H72a8,8,0,1,1,0-16H216A8,8,0,0,1,224,216Zm24-80v24a8,8,0,0,1-8,8H61.07a39.75,39.75,0,0,1-38.31-28.51L8.69,92.6A16,16,0,0,1,24,72h8a8,8,0,0,1,5.65,2.34L59.32,96H81.81l-9-26.94A16,16,0,0,1,88,48h8a8,8,0,0,1,5.66,2.34L147.32,96H208A40,40,0,0,1,248,136Zm-16,0a24,24,0,0,0-24-24H144a8,8,0,0,1-5.65-2.34L92.69,64H88l12.49,37.47A8,8,0,0,1,92.91,112H56a8,8,0,0,1-5.66-2.34L28.69,88H24l14.07,46.9a23.85,23.85,0,0,0,23,17.1H232Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M224,216a8,8,0,0,1-8,8H72a8,8,0,1,1,0-16H216A8,8,0,0,1,224,216ZM208,96H147.32L101.66,50.34A8,8,0,0,0,96,48H88A16,16,0,0,0,72.83,69.06l9,26.94H59.32L37.66,74.34A8,8,0,0,0,32,72H24A16,16,0,0,0,8.69,92.6l14.07,46.89A39.75,39.75,0,0,0,61.07,168H240a8,8,0,0,0,8-8V136A40,40,0,0,0,208,96Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M222,216a6,6,0,0,1-6,6H72a6,6,0,0,1,0-12H216A6,6,0,0,1,222,216Zm24-80v24a6,6,0,0,1-6,6H61.07a37.77,37.77,0,0,1-36.4-27.08L10.6,92A14,14,0,0,1,24,74h8a6,6,0,0,1,4.24,1.76L58.49,98h26.1L74.73,68.43A14,14,0,0,1,88,50h8a6,6,0,0,1,4.25,1.76L146.49,98H208A38,38,0,0,1,246,136Zm-12,0a26,26,0,0,0-26-26H144a6,6,0,0,1-4.24-1.76L93.52,62H88a2,2,0,0,0-1.9,2.63L98.6,102.1a6,6,0,0,1-5.69,7.9H56a6,6,0,0,1-4.24-1.76L29.52,86H24a1.93,1.93,0,0,0-1.6.81,1.91,1.91,0,0,0-.31,1.76l14.06,46.9A25.86,25.86,0,0,0,61.07,154H234Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M224,216a8,8,0,0,1-8,8H72a8,8,0,1,1,0-16H216A8,8,0,0,1,224,216Zm24-80v24a8,8,0,0,1-8,8H61.07a39.75,39.75,0,0,1-38.31-28.51L8.69,92.6A16,16,0,0,1,24,72h8a8,8,0,0,1,5.65,2.34L59.32,96H81.81l-9-26.94A16,16,0,0,1,88,48h8a8,8,0,0,1,5.66,2.34L147.32,96H208A40,40,0,0,1,248,136Zm-16,0a24,24,0,0,0-24-24H144a8,8,0,0,1-5.65-2.34L92.69,64H88l12.49,37.47A8,8,0,0,1,92.91,112H56a8,8,0,0,1-5.66-2.34L28.69,88H24l14.07,46.9a23.85,23.85,0,0,0,23,17.1H232Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M220,216a4,4,0,0,1-4,4H72a4,4,0,0,1,0-8H216A4,4,0,0,1,220,216Zm24-80v24a4,4,0,0,1-4,4H61.06a35.79,35.79,0,0,1-34.48-25.66L12.52,91.45A12,12,0,0,1,24,76h8a4,4,0,0,1,2.83,1.17L57.66,100h29.7L76.63,67.79A12,12,0,0,1,88,52h8a4,4,0,0,1,2.83,1.17L145.66,100H208A36,36,0,0,1,244,136Zm-8,0a28,28,0,0,0-28-28H144a4,4,0,0,1-2.83-1.17L94.35,60H88a4,4,0,0,0-3.8,5.26L96.7,102.74A4,4,0,0,1,92.91,108H56a4,4,0,0,1-2.82-1.17L30.35,84H24a4,4,0,0,0-3.83,5.15l14.07,46.9A27.83,27.83,0,0,0,61.06,156H236Z"/> }.into_view()
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
