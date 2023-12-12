//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Ear(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M220,104a12,12,0,0,1-24,0,68,68,0,0,0-136,0c0,25,7.58,32.3,16.35,40.76S96,163.71,96,188a32,32,0,0,0,32,32c9,0,16.19-3.7,22.75-11.64a12,12,0,0,1,18.5,15.28C158.09,237.15,144.21,244,128,244a56.06,56.06,0,0,1-56-56c0-14.09-4.63-18.56-12.31-26C49.13,151.86,36,139.19,36,104a92,92,0,0,1,184,0Zm-40.13,53.61a12,12,0,0,0-16.4,4.38,4,4,0,0,1-7.47-2c0-7.61,3.65-12.86,9.6-20.8C172,130.65,180,120,180,104a52,52,0,0,0-104,0,12,12,0,0,0,24,0,28,28,0,0,1,56,0c0,7.61-3.65,12.86-9.6,20.8C140,133.35,132,144,132,160a28,28,0,0,0,52.25,14A12,12,0,0,0,179.87,157.61Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,104c0,40-14.44,72-48,112-8.07,9.77-18.34,16-32,16a44,44,0,0,1-44-44c0-41.49-36-28-36-84a80,80,0,0,1,160,0Z" opacity="0.2"/><path d="M216,104a8,8,0,0,1-16,0,72,72,0,0,0-144,0c0,26.7,8.53,34.92,17.57,43.64C82.21,156,92,165.41,92,188a36,36,0,0,0,36,36c10.24,0,18.45-4.16,25.83-13.09a8,8,0,1,1,12.34,10.18C155.81,233.64,143,240,128,240a52.06,52.06,0,0,1-52-52c0-15.79-5.68-21.27-13.54-28.84C52.46,149.5,40,137.5,40,104a88,88,0,0,1,176,0Zm-38.13,57.08A8,8,0,0,0,166.93,164,8,8,0,0,1,152,160c0-9.33,4.82-15.76,10.4-23.2,6.37-8.5,13.6-18.13,13.6-32.8a48,48,0,0,0-96,0,8,8,0,0,0,16,0,32,32,0,0,1,64,0c0,9.33-4.82,15.76-10.4,23.2-6.37,8.5-13.6,18.13-13.6,32.8a24,24,0,0,0,44.78,12A8,8,0,0,0,177.87,161.08Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm20,128a4.21,4.21,0,0,0,1.33-.22,8,8,0,0,1,5.34,15.08A20,20,0,0,1,128,148c0-8.85,4.77-15.23,9-20.87,3.77-5,7-9.38,7-15.13a16,16,0,0,0-32,0,8,8,0,0,1-16,0,32,32,0,0,1,64,0c0,11.07-5.66,18.63-10.2,24.71-3.6,4.81-5.8,7.93-5.8,11.29A4,4,0,0,0,148,152Zm36-32a8,8,0,0,1-8-8,48,48,0,0,0-96,0c0,11.9,6.71,20.5,13.82,29.6,7,8.92,14.18,18.15,14.18,30.4a20,20,0,0,0,34,14.29,8,8,0,1,1,11.19,11.42A36,36,0,0,1,92,172c0-6.74-5-13.14-10.79-20.55C73.54,141.63,64,129.41,64,112a64,64,0,0,1,128,0A8,8,0,0,1,184,120Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M214,104a6,6,0,0,1-12,0,74,74,0,0,0-148,0c0,27.55,8.83,36.06,18.18,45.08,4.31,4.15,8.77,8.45,12.16,14.47C88.15,170.31,90,178.3,90,188a38,38,0,0,0,38,38c10.74,0,19.69-4.52,27.37-13.82a6,6,0,0,1,9.26,7.64C154.66,231.88,142.34,238,128,238a50.06,50.06,0,0,1-50-50c0-16.64-6.24-22.66-14.15-30.29C54.12,148.33,42,136.64,42,104a86,86,0,0,1,172,0Zm-37.14,58.81a6,6,0,0,0-8.19,2.19A10,10,0,0,1,150,160c0-10,5-16.67,10.8-24.4C167,127.35,174,118,174,104a46,46,0,0,0-92,0,6,6,0,0,0,12,0,34,34,0,0,1,68,0c0,10-5,16.67-10.8,24.4C145,136.65,138,146,138,160a22,22,0,0,0,41.05,11A6,6,0,0,0,176.86,162.81Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M216,104a8,8,0,0,1-16,0,72,72,0,0,0-144,0c0,26.7,8.53,34.92,17.57,43.64C82.21,156,92,165.41,92,188a36,36,0,0,0,36,36c10.24,0,18.45-4.16,25.83-13.09a8,8,0,1,1,12.34,10.18C155.81,233.64,143,240,128,240a52.06,52.06,0,0,1-52-52c0-15.79-5.68-21.27-13.54-28.84C52.46,149.5,40,137.5,40,104a88,88,0,0,1,176,0Zm-38.13,57.08A8,8,0,0,0,166.93,164,8,8,0,0,1,152,160c0-9.33,4.82-15.76,10.4-23.2,6.37-8.5,13.6-18.13,13.6-32.8a48,48,0,0,0-96,0,8,8,0,0,0,16,0,32,32,0,0,1,64,0c0,9.33-4.82,15.76-10.4,23.2-6.37,8.5-13.6,18.13-13.6,32.8a24,24,0,0,0,44.78,12A8,8,0,0,0,177.87,161.08Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M212,104a4,4,0,0,1-8,0,76,76,0,0,0-152,0c0,28.4,9.55,37.61,18.79,46.52C79.25,158.67,88,167.11,88,188a40,40,0,0,0,40,40c11.37,0,20.83-4.76,28.92-14.55a4,4,0,0,1,6.16,5.1C153.52,230.13,141.71,236,128,236a48.05,48.05,0,0,1-48-48c0-17.49-6.84-24.09-14.76-31.72C55.28,146.68,44,135.79,44,104a84,84,0,0,1,168,0Zm-36.14,60.54A4,4,0,0,0,170.4,166a12,12,0,0,1-22.4-6c0-10.67,5.44-17.92,11.2-25.6C165.49,126,172,117.33,172,104a44,44,0,0,0-88,0,4,4,0,0,0,8,0,36,36,0,0,1,72,0c0,10.67-5.44,17.92-11.2,25.6C146.51,138,140,146.67,140,160a20,20,0,0,0,37.32,10A4,4,0,0,0,175.86,164.54Z"/> }.into_view()
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
