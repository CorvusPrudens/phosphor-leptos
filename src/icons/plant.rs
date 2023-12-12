//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Plant(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M251.62,39.65a12,12,0,0,0-11.27-11.27c-53.28-3.14-96.2,13.37-114.84,44.14-12.14,20-12.56,44.17-1.46,67.3a75.21,75.21,0,0,0-11.24,20l-9.7-9.7c7.19-16.77,6.43-34.11-2.4-48.69C86.73,78.36,54.89,66,15.55,68.28A12,12,0,0,0,4.28,79.55C2,118.89,14.36,150.73,37.45,164.71a49.76,49.76,0,0,0,26,7.27,57.54,57.54,0,0,0,22.7-4.87L108,189v27a12,12,0,0,0,24,0V186.51a51.63,51.63,0,0,1,9.49-29.95A76.81,76.81,0,0,0,173.58,164a64.92,64.92,0,0,0,33.9-9.46C238.25,135.85,254.76,92.92,251.62,39.65ZM49.88,144.18c-13.19-8-21.18-27.46-21.83-52.13,24.67.65,44.14,8.65,52.13,21.83a26,26,0,0,1,3.63,17L72.48,119.51a12,12,0,0,0-17,17l11.34,11.34A26.27,26.27,0,0,1,49.88,144.18ZM195.05,134c-10.66,6.45-23,7.67-35.81,3.76l37.25-37.24a12,12,0,0,0-17-17l-37.25,37.24C138.37,108,139.59,95.61,146,85c12.77-21.09,43-33.07,82-33C228.14,91,216.14,121.18,195.05,134Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M138.54,141.46C106.62,88.25,149.18,35.05,239.63,40.37,245,130.82,191.75,173.39,138.54,141.46ZM16.26,80.26c-3.8,64.61,34.21,95,72.21,72.21C111.27,114.47,80.87,76.46,16.26,80.26Z" opacity="0.2"/><path d="M247.63,39.89a8,8,0,0,0-7.52-7.52c-51.76-3-93.32,12.74-111.18,42.22-11.8,19.48-11.78,43.16-.16,65.74a71.37,71.37,0,0,0-14.17,26.95L98.33,151c7.82-16.33,7.52-33.36-1-47.49C84.09,81.73,53.62,70,15.79,72.27a8,8,0,0,0-7.52,7.52c-2.23,37.83,9.46,68.3,31.25,81.5A45.82,45.82,0,0,0,63.44,168,54.58,54.58,0,0,0,87,162.33l25,25V216a8,8,0,0,0,16,0V186.51a55.61,55.61,0,0,1,12.27-35,73.91,73.91,0,0,0,33.31,8.4,60.9,60.9,0,0,0,31.83-8.86C234.89,133.21,250.67,91.65,247.63,39.89ZM86.06,138.74l-24.41-24.4a8,8,0,0,0-11.31,11.31l24.41,24.41c-9.61,3.18-18.93,2.39-26.94-2.46C32.47,138.31,23.79,116.32,24,88c28.31-.25,50.31,8.47,59.6,23.81C88.45,119.82,89.24,129.14,86.06,138.74Zm111.06-1.36c-13.4,8.11-29.15,8.73-45.15,2l53.69-53.7a8,8,0,0,0-11.31-11.32L140.65,128c-6.76-16-6.15-31.76,2-45.15,13.94-23,47-35.8,89.33-34.83C232.94,90.34,220.14,123.44,197.12,137.38Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M205.41,151.07a60.9,60.9,0,0,1-31.83,8.86,71.71,71.71,0,0,1-27.36-5.66A55.55,55.55,0,0,0,136,186.51V216a8,8,0,0,1-8.53,8,8.18,8.18,0,0,1-7.47-8.25V203.31L81.38,164.69A52.5,52.5,0,0,1,63.44,168a45.82,45.82,0,0,1-23.92-6.67C17.73,148.09,6,117.62,8.27,79.79a8,8,0,0,1,7.52-7.52c37.83-2.23,68.3,9.46,81.5,31.25A46,46,0,0,1,103.74,132a4,4,0,0,1-6.89,2.43l-19.2-20.1a8,8,0,0,0-11.31,11.31l53.88,55.25c.06-.78.13-1.56.21-2.33a68.56,68.56,0,0,1,18.64-39.46l50.59-53.46a8,8,0,0,0-11.31-11.32l-49,51.82a4,4,0,0,1-6.78-1.74c-4.74-17.48-2.65-34.88,6.4-49.82,17.86-29.48,59.42-45.26,111.18-42.22a8,8,0,0,1,7.52,7.52C250.67,91.65,234.89,133.21,205.41,151.07Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M245.63,40A6,6,0,0,0,240,34.37c-51-3-91.88,12.42-109.35,41.26-11.63,19.21-11.38,42.68.53,65a69.42,69.42,0,0,0-15.48,30.59L95.9,151.41c8.15-16.1,8.1-32.95-.32-46.85C82.77,83.41,53,72.07,15.9,74.27a6,6,0,0,0-5.63,5.64C8.09,117,19.41,146.77,40.56,159.58A43.75,43.75,0,0,0,63.44,166a53.17,53.17,0,0,0,24-6L114,186.48V216a6,6,0,0,0,12,0V186.51A57.55,57.55,0,0,1,139.76,149c11.21,5.93,22.72,8.91,33.82,8.91a58.9,58.9,0,0,0,30.79-8.57C233.21,131.89,248.63,91,245.63,40ZM46.78,149.31C30.47,139.44,21.39,116,22,86c30-.65,53.41,8.44,63.28,24.75,5.68,9.37,6.16,20.38,1.54,31.59L60.24,115.75a6,6,0,0,0-8.49,8.49l26.62,26.61C67.16,155.47,56.15,155,46.78,149.31Zm151.38-10.22c-14.74,8.92-32.14,9.18-49.67.9l55.76-55.75a6,6,0,0,0-8.49-8.49L140,131.51c-8.28-17.53-8-34.93.9-49.66,14.52-24,49.06-37.18,93-35.75C235.33,90,222.14,124.56,198.16,139.09Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M247.63,39.89a8,8,0,0,0-7.52-7.52c-51.76-3-93.32,12.74-111.18,42.22-11.8,19.49-11.78,43.16-.16,65.74a71.34,71.34,0,0,0-14.17,27L98.33,151c7.82-16.33,7.52-33.35-1-47.49-13.2-21.79-43.67-33.47-81.5-31.25a8,8,0,0,0-7.52,7.52c-2.23,37.83,9.46,68.3,31.25,81.5A45.82,45.82,0,0,0,63.44,168,54.58,54.58,0,0,0,87,162.33l25,25V216a8,8,0,0,0,16,0V186.51a55.61,55.61,0,0,1,12.27-35,73.91,73.91,0,0,0,33.31,8.4,60.9,60.9,0,0,0,31.83-8.86C234.89,133.21,250.67,91.65,247.63,39.89ZM47.81,147.6C32.47,138.31,23.79,116.32,24,88c28.32-.24,50.31,8.47,59.6,23.81,4.85,8,5.64,17.33,2.46,26.94L61.65,114.34a8,8,0,0,0-11.31,11.31l24.41,24.41C65.14,153.24,55.82,152.45,47.81,147.6Zm149.31-10.22c-13.4,8.11-29.15,8.73-45.15,2l53.69-53.7a8,8,0,0,0-11.31-11.31L140.65,128c-6.76-16-6.15-31.76,2-45.15,13.94-23,47-35.82,89.33-34.83C232.94,90.34,220.14,123.44,197.12,137.38Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M243.63,40.12a4,4,0,0,0-3.76-3.75c-50.25-3-90.44,12.1-107.52,40.29-11.64,19.22-11.17,41.92,1.24,64.21a67.33,67.33,0,0,0-16.65,34.41L93.45,151.79c8.63-16,8.81-32.33.42-46.19C81.45,85.09,52.35,74.13,16,76.26A4,4,0,0,0,12.26,80c-2.13,36.33,8.83,65.43,29.34,77.85a41.82,41.82,0,0,0,21.88,6.18,51.29,51.29,0,0,0,24.31-6.6L116,185.67c0,.28,0,.56,0,.84V216a4,4,0,0,0,8,0V186.51a59.57,59.57,0,0,1,15.29-40c11.49,6.36,23.07,9.56,34.24,9.56a57,57,0,0,0,29.81-8.41C231.52,130.57,246.59,90.38,243.63,40.12ZM45.74,151C28.47,140.56,19,115.69,20.08,84.08,51.69,83,76.56,92.47,87,109.74c6.48,10.71,6.59,23.37.46,36.09L58.82,117.17a4,4,0,0,0-5.65,5.65l28.67,28.67C69.11,157.62,56.45,157.51,45.74,151ZM199.19,140.8c-16.05,9.72-35.09,9.59-54.08-.25l57.72-57.73a4,4,0,0,0-5.65-5.66l-57.73,57.73c-9.84-19-10-38-.25-54.08,15.11-25,51.08-38.53,96.63-36.64C237.73,89.72,224.15,125.69,199.19,140.8Z"/> }.into_view()
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
