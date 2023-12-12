//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Crown(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M246.46,73.17a16,16,0,0,0-17.74-2.26l-46.9,23.38-40-66.49a16.11,16.11,0,0,0-27.6,0l-40,66.49L27.31,70.92A16.1,16.1,0,0,0,4.82,90.35l37,113.35a12,12,0,0,0,17.51,6.61C59.57,210.17,84.39,196,128,196s68.43,14.19,68.62,14.3a12,12,0,0,0,17.57-6.58l37-113.29A16,16,0,0,0,246.46,73.17ZM195.53,183.52C182.18,178.4,159.2,172,128,172s-54.18,6.42-67.53,11.54l-27-82.71L70,119a16.18,16.18,0,0,0,21-6.11l37-61.49,37,61.5a16.18,16.18,0,0,0,21,6.1l36.52-18.2Zm-19.67-31A12,12,0,0,1,164,162.69a12.91,12.91,0,0,1-1.85-.14,229.32,229.32,0,0,0-68.34,0,12,12,0,0,1-3.66-23.72,253.38,253.38,0,0,1,75.66,0A12,12,0,0,1,175.86,152.52Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M239.78,86.62,202.78,200S176,184,128,184s-74.78,16-74.78,16l-37-113.37a4.1,4.1,0,0,1,5.72-5l53.41,26.62a4.11,4.11,0,0,0,5.36-1.56L124.48,34a4.11,4.11,0,0,1,7,0l43.77,72.74a4.12,4.12,0,0,0,5.35,1.56l53.43-26.64A4.1,4.1,0,0,1,239.78,86.62Z" opacity="0.2"/><path d="M243.84,76.19a12.08,12.08,0,0,0-13.34-1.7l-50.21,25L138.37,29.86a12.11,12.11,0,0,0-20.74,0L75.71,99.52l-50.19-25A12.11,12.11,0,0,0,8.62,89.12l37,113.36a8,8,0,0,0,11.68,4.4C57.55,206.73,83.12,192,128,192s70.45,14.73,70.68,14.87a8,8,0,0,0,11.71-4.39l37-113.33A12.06,12.06,0,0,0,243.84,76.19ZM198,188.83C186,183.74,162.08,176,128,176s-58,7.74-70,12.83L26.71,93l45.07,22.47a12.17,12.17,0,0,0,15.78-4.59L128,43.66l40.44,67.2a12.18,12.18,0,0,0,15.77,4.59L229.29,93Zm-22.13-32a8,8,0,0,1-7.87,6.61,8.36,8.36,0,0,1-1.4-.12,228.2,228.2,0,0,0-77.22,0,8,8,0,0,1-2.78-15.76,244.42,244.42,0,0,1,82.78,0A8,8,0,0,1,175.88,156.8Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M243.84,76.19a12.08,12.08,0,0,0-13.34-1.7l-50.21,25L138.37,29.86a12.11,12.11,0,0,0-20.74,0L75.71,99.52l-50.19-25A12.11,12.11,0,0,0,8.62,89.12l37,113.36a8,8,0,0,0,11.68,4.4C57.55,206.73,83.12,192,128,192s70.45,14.73,70.68,14.87a8,8,0,0,0,11.71-4.39l37-113.33A12.06,12.06,0,0,0,243.84,76.19Zm-68,80.61a8,8,0,0,1-7.87,6.61,8.36,8.36,0,0,1-1.4-.12,228.2,228.2,0,0,0-77.22,0,8,8,0,0,1-2.78-15.76,244.42,244.42,0,0,1,82.78,0A8,8,0,0,1,175.88,156.8Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M242.52,77.7a10.07,10.07,0,0,0-11.12-1.42l-51.87,25.86L136.66,30.89a10.11,10.11,0,0,0-17.32,0L76.47,102.14,24.62,76.29A10.1,10.1,0,0,0,10.52,88.5l37,113.36a6,6,0,0,0,8.77,3.3c.07,0,6.56-3.84,18.6-7.58C86,194.12,104.21,190,128,190s42,4.12,53.12,7.58c12,3.74,18.53,7.54,18.58,7.57a6,6,0,0,0,8.78-3.29l37-113.34A10,10,0,0,0,242.52,77.7ZM199.23,191.53c-11-4.92-35.4-13.53-71.23-13.53s-60.23,8.61-71.23,13.53L23.32,89.05l49.35,24.6a10.16,10.16,0,0,0,13.18-3.83l42.15-70,42.15,70.05a10.17,10.17,0,0,0,13.17,3.83l49.36-24.61Zm-25.32-35.08a6,6,0,0,1-5.9,5,6.2,6.2,0,0,1-1-.09,230.26,230.26,0,0,0-77.92,0A6,6,0,0,1,87,149.5a242.36,242.36,0,0,1,82.08,0A6,6,0,0,1,173.91,156.45Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M243.84,76.19a12.08,12.08,0,0,0-13.34-1.7l-50.21,25L138.37,29.86a12.11,12.11,0,0,0-20.74,0L75.71,99.52l-50.19-25A12.11,12.11,0,0,0,8.62,89.12l37,113.36a8,8,0,0,0,11.68,4.4C57.55,206.73,83.12,192,128,192s70.45,14.73,70.68,14.87a8,8,0,0,0,11.71-4.39l37-113.33A12.06,12.06,0,0,0,243.84,76.19ZM198,188.83C186,183.74,162.08,176,128,176s-58,7.74-70,12.83L26.71,93l45.07,22.47a12.17,12.17,0,0,0,15.78-4.59L128,43.66l40.44,67.2a12.18,12.18,0,0,0,15.77,4.59L229.29,93Zm-22.13-32a8,8,0,0,1-7.87,6.61,8.36,8.36,0,0,1-1.4-.12,228.2,228.2,0,0,0-77.22,0,8,8,0,0,1-2.78-15.76,244.42,244.42,0,0,1,82.78,0A8,8,0,0,1,175.88,156.8Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M241.21,79.21a8.07,8.07,0,0,0-8.92-1.14l-53.43,26.64a.11.11,0,0,1-.14,0L135,31.93a8.11,8.11,0,0,0-13.9,0L77.28,104.66a.11.11,0,0,1-.15,0L23.73,78.08a8.1,8.1,0,0,0-11.31,9.8l37,113.36a4,4,0,0,0,5.85,2.2C55.52,203.28,81.83,188,128,188s72.47,15.28,72.73,15.43a4,4,0,0,0,5.85-2.19l37-113.36A8.06,8.06,0,0,0,241.21,79.21ZM200.43,194.29a125.3,125.3,0,0,0-15.84-6.11C172.74,184.45,153.3,180,128,180s-44.74,4.45-56.6,8.18a126.55,126.55,0,0,0-15.84,6.11L20,85.32a.14.14,0,0,1,.13-.08h0l53.4,26.62a8.14,8.14,0,0,0,10.57-3.07l43.78-72.74a.1.1,0,0,1,.18,0l43.78,72.74a8.15,8.15,0,0,0,10.56,3.08L235.9,85.21a.13.13,0,0,1,.08.16Zm-28.49-38.18a4,4,0,0,1-3.93,3.3,4,4,0,0,1-.7-.06,232.31,232.31,0,0,0-78.62,0,4,4,0,0,1-1.39-7.88,240.42,240.42,0,0,1,81.4,0A4,4,0,0,1,171.94,156.11Z"/> }.into_view()
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
