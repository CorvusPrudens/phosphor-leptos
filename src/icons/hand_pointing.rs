//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn HandPointing(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M188,84a31.94,31.94,0,0,0-11.22,2A32,32,0,0,0,140,69V44a32,32,0,0,0-64,0v66.83A32,32,0,0,0,24.25,148l4.68,8.24C63.11,216.48,78.72,244,128,244a92.1,92.1,0,0,0,92-92V116A32,32,0,0,0,188,84Zm8,68a68.08,68.08,0,0,1-68,68c-34,0-43.49-14.45-78.2-75.65l-4.69-8.28a.16.16,0,0,1,0-.07,8,8,0,0,1,13.86-8c.06.12.13.23.2.35l18.68,30A12,12,0,0,0,100,152V44a8,8,0,0,1,16,0v68a12,12,0,0,0,24,0V100a8,8,0,0,1,16,0v20a12,12,0,0,0,24,0v-4a8,8,0,0,1,16,0Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,116v36a80,80,0,0,1-80,80c-44.18,0-55.81-24-93.32-90a20,20,0,0,1,34.64-20L88,152V44a20,20,0,0,1,40,0v56a20,20,0,0,1,40,0v16a20,20,0,0,1,40,0Z" opacity="0.2"/><path d="M188,88a27.86,27.86,0,0,0-13.35,3.39A28,28,0,0,0,136,74.7V44a28,28,0,0,0-56,0v80l-3.82-6.13A28,28,0,0,0,27.73,146l4.67,8.23C66.81,214.89,81.05,240,128,240a88.1,88.1,0,0,0,88-88V116A28,28,0,0,0,188,88Zm12,64a72.08,72.08,0,0,1-72,72c-37.63,0-47.84-18-81.68-77.68l-4.69-8.27,0-.05A12,12,0,0,1,46,121.61a11.88,11.88,0,0,1,6-1.6,12,12,0,0,1,10.41,6,1.76,1.76,0,0,0,.14.23l18.67,30A8,8,0,0,0,96,152V44a12,12,0,0,1,24,0v68a8,8,0,0,0,16,0V100a12,12,0,0,1,24,0v20a8,8,0,0,0,16,0v-4a12,12,0,0,1,24,0Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M216,104v50.93c0,46.2-36.85,84.55-83,85.06A83.71,83.71,0,0,1,72.6,215.4C50.79,192.33,26.15,136,26.15,136a16,16,0,0,1,6.53-22.23c7.66-4,17.1-.84,21.4,6.62l21,36.44a6.09,6.09,0,0,0,6,3.09l.12,0A8.19,8.19,0,0,0,88,151.74V32a16,16,0,0,1,16.77-16c8.61.4,15.23,7.82,15.23,16.43V104a8,8,0,0,0,8.53,8,8.17,8.17,0,0,0,7.47-8.25V88a16,16,0,0,1,16.77-16c8.61.4,15.23,7.82,15.23,16.43V112a8,8,0,0,0,8.53,8,8.17,8.17,0,0,0,7.47-8.25v-7.28c0-8.61,6.62-16,15.23-16.43A16,16,0,0,1,216,104Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M188,90a25.87,25.87,0,0,0-14.59,4.49A26,26,0,0,0,134,78.1V44a26,26,0,0,0-52,0v87l-7.53-12.1a26,26,0,0,0-45,26.07l4.67,8.25c34,60,48.07,84.77,93.86,84.77a86.1,86.1,0,0,0,86-86V116A26,26,0,0,0,188,90Zm14,62a74.09,74.09,0,0,1-74,74c-38.8,0-50-19.83-83.42-78.69L39.89,139l0,0A14,14,0,0,1,45,119.88,13.87,13.87,0,0,1,52,118a14,14,0,0,1,12.15,7l.1.17,18.68,30A6,6,0,0,0,94,152V44a14,14,0,0,1,28,0v68a6,6,0,0,0,12,0V100a14,14,0,0,1,28,0v20a6,6,0,0,0,12,0v-4a14,14,0,0,1,28,0Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M188,88a27.86,27.86,0,0,0-13.35,3.39A28,28,0,0,0,136,74.7V44a28,28,0,0,0-56,0v80l-3.82-6.13A28,28,0,0,0,27.73,146l4.67,8.23C66.81,214.89,81.05,240,128,240a88.1,88.1,0,0,0,88-88V116A28,28,0,0,0,188,88Zm12,64a72.08,72.08,0,0,1-72,72c-37.63,0-47.84-18-81.68-77.68l-4.69-8.27,0-.05A12,12,0,0,1,46,121.61a11.88,11.88,0,0,1,6-1.6,12,12,0,0,1,10.41,6,1.76,1.76,0,0,0,.14.23l18.67,30A8,8,0,0,0,96,152V44a12,12,0,0,1,24,0v68a8,8,0,0,0,16,0V100a12,12,0,0,1,24,0v20a8,8,0,0,0,16,0v-4a12,12,0,0,1,24,0Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M188,92a23.88,23.88,0,0,0-16.07,6.19A24,24,0,0,0,132,82.13V44a24,24,0,0,0-48,0v94L72.75,119.94A24,24,0,0,0,31.2,144l4.68,8.25C53.21,182.8,64.66,203,77.66,216.33,91.28,230.3,105.86,236,128,236a84.09,84.09,0,0,0,84-84V116A24,24,0,0,0,188,92Zm16,60a76.09,76.09,0,0,1-76,76c-40,0-51.35-20.08-85.16-79.71L38.15,140v0a16,16,0,0,1,27.71-16,.75.75,0,0,1,.07.12l18.68,30A4,4,0,0,0,92,152V44a16,16,0,0,1,32,0v68a4,4,0,0,0,8,0V100a16,16,0,0,1,32,0v20a4,4,0,0,0,8,0v-4a16,16,0,0,1,32,0Z"/> }.into_view()
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
