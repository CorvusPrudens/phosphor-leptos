//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn GasCan(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M200,20H131.31a19.86,19.86,0,0,0-14.14,5.86L105.66,37.37,90.14,21.86a20,20,0,0,0-28.28,0l-24,24a20,20,0,0,0,0,28.28L53.37,89.66,41.86,101.17A19.86,19.86,0,0,0,36,115.31V216a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V40A20,20,0,0,0,200,20ZM57.66,60,76,41.66,88.69,54.34,70.34,72.69ZM196,212H60V117L78.83,98.14h0L133,44h63ZM136,68a12,12,0,0,1,12-12h20a12,12,0,0,1,0,24H148A12,12,0,0,1,136,68Zm39.5,65.37L147.21,156l28.29,22.63a12,12,0,0,1-15,18.74l-32.5-26-32.5,26a12,12,0,0,1-15-18.74L108.79,156,80.5,133.37a12,12,0,0,1,15-18.74l32.5,26,32.5-26a12,12,0,0,1,15,18.74Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M208,40V216a8,8,0,0,1-8,8H56a8,8,0,0,1-8-8V107.31a8,8,0,0,1,2.34-5.65l67.32-67.32A8,8,0,0,1,123.31,32H200A8,8,0,0,1,208,40Z" opacity="0.2"/><path d="M200,24H123.31A15.86,15.86,0,0,0,112,28.69L101.66,39,91.31,28.69a16,16,0,0,0-22.62,0l-24,24a16,16,0,0,0,0,22.62L55,85.66,44.69,96A15.86,15.86,0,0,0,40,107.31V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM56,64,80,40,90.34,50.34l-24,24ZM200,216H56V107.31l16-16h0L123.31,40H200ZM128,64a8,8,0,0,1,8-8h40a8,8,0,0,1,0,16H136A8,8,0,0,1,128,64Zm52.8,62.4L141.33,156l39.47,29.6a8,8,0,1,1-9.6,12.8L128,166,84.8,198.4a8,8,0,0,1-9.6-12.8L114.67,156,75.2,126.4a8,8,0,0,1,9.6-12.8L128,146l43.2-32.4a8,8,0,0,1,9.6,12.8Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M200,24H123.31A15.86,15.86,0,0,0,112,28.69L101.66,39,91.31,28.69a16,16,0,0,0-22.62,0l-24,24a16,16,0,0,0,0,22.62L55,85.66,44.69,96A15.86,15.86,0,0,0,40,107.31V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM56,64,80,40,90.34,50.34l-24,24ZM180.8,185.6a8,8,0,1,1-9.6,12.8L128,166,84.8,198.4a8,8,0,0,1-9.6-12.8L114.67,156,75.2,126.4a8,8,0,0,1,9.6-12.8L128,146l43.2-32.4a8,8,0,0,1,9.6,12.8L141.33,156ZM176,72H136a8,8,0,0,1,0-16h40a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M200,26H123.31a13.94,13.94,0,0,0-9.9,4.1L101.66,41.86,89.9,30.1a14,14,0,0,0-19.8,0l-24,24a14,14,0,0,0,0,19.8L57.86,85.66,46.1,97.41a13.94,13.94,0,0,0-4.1,9.9V216a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V40A14,14,0,0,0,200,26ZM54.59,65.41a2,2,0,0,1,0-2.82l24-24a2,2,0,0,1,2.82,0L93.17,50.34,66.34,77.17ZM202,216a2,2,0,0,1-2,2H56a2,2,0,0,1-2-2V107.31a2,2,0,0,1,.59-1.41l16-16h0L105.9,54.59h0l16-16a2,2,0,0,1,1.41-.59H200a2,2,0,0,1,2,2ZM182,64a6,6,0,0,1-6,6H136a6,6,0,0,1,0-12h40A6,6,0,0,1,182,64Zm-2.4,60.8L138,156l41.6,31.2a6,6,0,1,1-7.2,9.6L128,163.5,83.6,196.8a6,6,0,0,1-7.2-9.6L118,156,76.4,124.8a6,6,0,0,1,7.2-9.6L128,148.5l44.4-33.3a6,6,0,1,1,7.2,9.6Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M200,24H123.31A15.86,15.86,0,0,0,112,28.69L101.66,39,91.31,28.69a16,16,0,0,0-22.62,0l-24,24a16,16,0,0,0,0,22.62L55,85.66,44.69,96A15.86,15.86,0,0,0,40,107.31V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM56,64,80,40,90.34,50.34l-24,24ZM200,216H56V107.31l16-16h0L123.31,40H200ZM128,64a8,8,0,0,1,8-8h40a8,8,0,0,1,0,16H136A8,8,0,0,1,128,64Zm52.8,62.4L141.33,156l39.47,29.6a8,8,0,1,1-9.6,12.8L128,166,84.8,198.4a8,8,0,0,1-9.6-12.8L114.67,156,75.2,126.4a8,8,0,0,1,9.6-12.8L128,146l43.2-32.4a8,8,0,0,1,9.6,12.8Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M200,28H123.31a12,12,0,0,0-8.48,3.51L101.66,44.69,88.49,31.51a12,12,0,0,0-17,0l-24,24a12,12,0,0,0,0,17L60.69,85.66,47.52,98.83A11.9,11.9,0,0,0,44,107.31V216a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V40A12,12,0,0,0,200,28ZM53.17,66.83a4,4,0,0,1,0-5.66l24-24a4,4,0,0,1,5.66,0L96,50.34,66.34,80ZM204,216a4,4,0,0,1-4,4H56a4,4,0,0,1-4-4V107.31a4,4,0,0,1,1.17-2.82l16-16h0l35.31-35.31h0l16-16A4,4,0,0,1,123.31,36H200a4,4,0,0,1,4,4ZM180,64a4,4,0,0,1-4,4H136a4,4,0,0,1,0-8h40A4,4,0,0,1,180,64Zm-1.6,59.2L134.67,156l43.73,32.8a4,4,0,0,1-4.8,6.4L128,161,82.4,195.2a4,4,0,0,1-4.8-6.4L121.33,156,77.6,123.2a4,4,0,0,1,4.8-6.4L128,151l45.6-34.2a4,4,0,1,1,4.8,6.4Z"/> }.into_view()
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
