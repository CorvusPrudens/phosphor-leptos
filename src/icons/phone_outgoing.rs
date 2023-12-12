//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn PhoneOutgoing(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M151.52,104.48a12,12,0,0,1,0-17L179,60H168a12,12,0,0,1,0-24h40a12,12,0,0,1,12,12V88a12,12,0,0,1-24,0V77l-27.51,27.51a12,12,0,0,1-17,0Zm84.33,71.1A60.27,60.27,0,0,1,176,228C94.39,228,28,161.61,28,80A60.27,60.27,0,0,1,80.42,20.15,20.05,20.05,0,0,1,101.2,32l21.11,47.13a1.42,1.42,0,0,0,.08.18,20,20,0,0,1-1.66,18.83,10.67,10.67,0,0,1-.85,1.15L100.82,122c7.06,12.84,20.5,26.16,33.49,33.21l22.31-19a13.08,13.08,0,0,1,1.12-.84,19.91,19.91,0,0,1,19-1.74l.18.08L224,154.8A20.06,20.06,0,0,1,235.85,175.58Zm-24.31-.06-42-18.81-22.43,19.07a11.63,11.63,0,0,1-1.11.85A20,20,0,0,1,126.31,178c-19.48-9.4-38.89-28.68-48.31-48a20,20,0,0,1,1.28-19.64,10.75,10.75,0,0,1,.86-1.15L99.3,86.5l-18.82-42A36.29,36.29,0,0,0,52,80,124.15,124.15,0,0,0,176,204,36.29,36.29,0,0,0,211.54,175.52Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M223.94,174.08A48.33,48.33,0,0,1,176,216,136,136,0,0,1,40,80,48.33,48.33,0,0,1,81.92,32.06a8,8,0,0,1,8.3,4.8l21.13,47.2a8,8,0,0,1-.66,7.53L89.32,117a7.93,7.93,0,0,0-.54,7.81c8.27,16.93,25.77,34.22,42.75,42.41a7.92,7.92,0,0,0,7.83-.59l25-21.3a8,8,0,0,1,7.59-.69l47.16,21.13A8,8,0,0,1,223.94,174.08Z" opacity="0.2"/><path d="M154.34,101.66a8,8,0,0,1,0-11.32L188.69,56H168a8,8,0,0,1,0-16h40a8,8,0,0,1,8,8V88a8,8,0,0,1-16,0V67.31l-34.34,34.35a8,8,0,0,1-11.32,0Zm77.54,73.42A56.26,56.26,0,0,1,176,224C96.6,224,32,159.4,32,80A56.26,56.26,0,0,1,80.92,24.12a16,16,0,0,1,16.62,9.51l21.12,47.16,0,.12A16,16,0,0,1,117.39,96c-.18.27-.37.52-.57.77L96,121.45c7.49,15.22,23.41,31,38.83,38.51l24.34-20.71a8.12,8.12,0,0,1,.75-.56,15.93,15.93,0,0,1,15.17-1.4l.13.06,47.11,21.11A16,16,0,0,1,231.88,175.08Zm-15.88-2s-.07,0-.11,0h0l-47-21.06-24.35,20.72a8.44,8.44,0,0,1-.74.56,16,16,0,0,1-15.75,1.14c-18.73-9.05-37.4-27.58-46.46-46.11a16,16,0,0,1,1-15.7,6.13,6.13,0,0,1,.57-.77L104,87.15l-21-47a.61.61,0,0,1,0-.12A40.2,40.2,0,0,0,48,80,128.14,128.14,0,0,0,176,208,40.21,40.21,0,0,0,216,173.07Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M154.34,101.66a8,8,0,0,1,0-11.32L188.69,56H168a8,8,0,0,1,0-16h40a8,8,0,0,1,8,8V88a8,8,0,0,1-16,0V67.31l-34.34,34.35a8,8,0,0,1-11.32,0Zm68,56.8-47.11-21.11-.13-.06a16,16,0,0,0-15.17,1.4,8.12,8.12,0,0,0-.75.56L134.87,160c-15.42-7.49-31.34-23.29-38.83-38.51l20.78-24.71c.2-.25.39-.5.57-.77a16,16,0,0,0,1.32-15.06l0-.12L97.54,33.64a16,16,0,0,0-16.62-9.52A56.26,56.26,0,0,0,32,80c0,79.4,64.6,144,144,144a56.26,56.26,0,0,0,55.88-48.92A16,16,0,0,0,222.37,158.46Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M155.76,100.24a6,6,0,0,1,0-8.48L193.52,54H168a6,6,0,0,1,0-12h40a6,6,0,0,1,6,6V88a6,6,0,0,1-12,0V62.48l-37.76,37.76a6,6,0,0,1-8.48,0Zm74.13,74.59A54.25,54.25,0,0,1,176,222C97.7,222,34,158.3,34,80A54.24,54.24,0,0,1,81.17,26.11a14,14,0,0,1,14.56,8.38l21.1,47.11a14,14,0,0,1-1.12,13.27,6.13,6.13,0,0,1-.42.58l-21.07,25a1.91,1.91,0,0,0,0,1.68c7.66,15.68,24.1,32,40,39.65a1.88,1.88,0,0,0,1.68-.06l24.69-21a4.81,4.81,0,0,1,.56-.42,14,14,0,0,1,13.28-1.22l47.24,21.17A14,14,0,0,1,229.89,174.83ZM218,173.32a2,2,0,0,0-1.21-2l-47.25-21.17a1.92,1.92,0,0,0-1.6.1l-24.68,21c-.18.15-.37.29-.56.42a14,14,0,0,1-13.77,1c-18.36-8.87-36.66-27-45.53-45.19a14,14,0,0,1,.91-13.73,4.73,4.73,0,0,1,.43-.57l21.06-25.06a2,2,0,0,0,0-1.67L84.74,39.31A2,2,0,0,0,82.9,38h-.23A42.23,42.23,0,0,0,46,80c0,71.68,58.32,130,130,130A42.24,42.24,0,0,0,218,173.32Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M154.34,101.66a8,8,0,0,1,0-11.32L188.69,56H168a8,8,0,0,1,0-16h40a8,8,0,0,1,8,8V88a8,8,0,0,1-16,0V67.31l-34.34,34.35a8,8,0,0,1-11.32,0Zm77.54,73.42A56.26,56.26,0,0,1,176,224C96.6,224,32,159.4,32,80A56.26,56.26,0,0,1,80.92,24.12a16,16,0,0,1,16.62,9.51l21.12,47.16,0,.12A16,16,0,0,1,117.39,96c-.18.27-.37.52-.57.77L96,121.45c7.49,15.22,23.41,31,38.83,38.51l24.34-20.71a8.12,8.12,0,0,1,.75-.56,15.93,15.93,0,0,1,15.17-1.4l.13.06,47.11,21.11A16,16,0,0,1,231.88,175.08Zm-15.88-2s-.07,0-.11,0h0l-47-21.06-24.35,20.72a8.44,8.44,0,0,1-.74.56,16,16,0,0,1-15.75,1.14c-18.73-9.05-37.4-27.58-46.46-46.11a16,16,0,0,1,1-15.7,6.13,6.13,0,0,1,.57-.77L104,87.15l-21-47a.61.61,0,0,1,0-.12A40.2,40.2,0,0,0,48,80,128.14,128.14,0,0,0,176,208,40.21,40.21,0,0,0,216,173.07Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M157.17,98.83a4,4,0,0,1,0-5.66L198.34,52H168a4,4,0,0,1,0-8h40a4,4,0,0,1,4,4V88a4,4,0,0,1-8,0V57.66L162.83,98.83a4,4,0,0,1-5.66,0Zm70.74,75.75A52.25,52.25,0,0,1,176,220C98.8,220,36,157.2,36,80A52.25,52.25,0,0,1,81.42,28.09,12,12,0,0,1,93.9,35.28L115,82.42a12,12,0,0,1-1,11.36c-.09.13-.18.26-.28.38l-21.2,25.21a3.9,3.9,0,0,0-.18,3.69c7.84,16.05,24.65,32.73,40.89,40.57a3.9,3.9,0,0,0,3.7-.21L161.8,142.3a3.37,3.37,0,0,1,.38-.28A12,12,0,0,1,173.56,141l47.22,21.16A12,12,0,0,1,227.91,174.58Zm-10.35-5.12L170.35,148.3a3.93,3.93,0,0,0-3.57.27L142,169.69l-.37.28a12,12,0,0,1-11.79.87c-18-8.69-35.91-26.48-44.6-44.27A12,12,0,0,1,86,114.82c.09-.14.19-.26.29-.39l21.19-25.2a4,4,0,0,0,.23-3.6L86.57,38.49A4,4,0,0,0,82.9,36a3.87,3.87,0,0,0-.48,0A44.23,44.23,0,0,0,44,80c0,72.78,59.22,132,132,132a44.23,44.23,0,0,0,44-38.42A4,4,0,0,0,217.56,169.46Z"/> }.into_view()
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
