//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn CloudMoon(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M164,68c-1.66,0-3.31.06-4.95.16a75.93,75.93,0,0,0-58-62.23A12,12,0,0,0,86.68,20.31,52.05,52.05,0,0,1,24.3,82.68,12,12,0,0,0,9.93,97.07a76.61,76.61,0,0,0,27.91,43.27A56,56,0,0,0,84,228h80a80,80,0,0,0,0-160ZM111.89,36a51.64,51.64,0,0,1,23.68,37.17,80.39,80.39,0,0,0-45.18,43.15A56.5,56.5,0,0,0,84,116a55.69,55.69,0,0,0-28.23,7.66,52.69,52.69,0,0,1-15.63-15.77A76.11,76.11,0,0,0,111.89,36ZM164,204H84a32,32,0,0,1,0-64h.28c-.11,1.1-.2,2.2-.26,3.3a12,12,0,0,0,24,1.4,55.78,55.78,0,0,1,1.74-11l.15-.55A56.06,56.06,0,1,1,164,204Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M98.31,130.38ZM94.38,17.62h0A64.06,64.06,0,0,1,17.62,94.38h0A64.12,64.12,0,0,0,55,138.93h0a44.08,44.08,0,0,1,43.33-8.54,68.13,68.13,0,0,1,45.47-47.32l.15,0c0-1,.07-2,.07-3A64,64,0,0,0,94.38,17.62Z" opacity="0.2"/><path d="M164,72a76.45,76.45,0,0,0-12.36,1A71.93,71.93,0,0,0,96.17,9.83a8,8,0,0,0-9.59,9.58A56.45,56.45,0,0,1,88,32,56.06,56.06,0,0,1,32,88a56.45,56.45,0,0,1-12.59-1.42,8,8,0,0,0-9.59,9.59,72.22,72.22,0,0,0,32.29,45.06A52,52,0,0,0,84,224h80a76,76,0,0,0,0-152ZM29.37,104c.87,0,1.75,0,2.63,0a72.08,72.08,0,0,0,72-72c0-.89,0-1.78,0-2.67a55.63,55.63,0,0,1,32,48,76.28,76.28,0,0,0-43,43.4A52,52,0,0,0,54,129.59,56.22,56.22,0,0,1,29.37,104ZM164,208H84a36,36,0,1,1,4.78-71.69c-.37,2.37-.63,4.79-.77,7.23a8,8,0,0,0,16,.92,58.91,58.91,0,0,1,1.88-11.81c0-.16.09-.32.12-.48A60.06,60.06,0,1,1,164,208Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M164,72a76.35,76.35,0,0,0-12.36,1A71.93,71.93,0,0,0,96.17,9.83a8,8,0,0,0-9.59,9.58A56.45,56.45,0,0,1,88,32,56.06,56.06,0,0,1,32,88a56.45,56.45,0,0,1-12.59-1.42,8,8,0,0,0-9.59,9.59,72.22,72.22,0,0,0,32.29,45.06A52,52,0,0,0,84,224h80a76,76,0,0,0,0-152ZM29.37,104c.87,0,1.75,0,2.63,0a72.08,72.08,0,0,0,72-72c0-.89,0-1.78,0-2.67a55.64,55.64,0,0,1,32,48.05A76.4,76.4,0,0,0,93,120.78a52.38,52.38,0,0,0-9-.78,51.69,51.69,0,0,0-30,9.59A56.22,56.22,0,0,1,29.37,104Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M164,74a74.41,74.41,0,0,0-14.17,1.36A70,70,0,0,0,95.72,11.77,6,6,0,0,0,88.53,19,58.06,58.06,0,0,1,19,88.53a6,6,0,0,0-7.19,7.19,70.22,70.22,0,0,0,33.3,44.95A50,50,0,0,0,84,222h80a74,74,0,0,0,0-148ZM26.22,101.76Q29.1,102,32,102a70.08,70.08,0,0,0,70-70c0-1.94-.08-3.88-.24-5.8A57.64,57.64,0,0,1,138,78.71,74.32,74.32,0,0,0,94.2,123,50.36,50.36,0,0,0,84,122a49.74,49.74,0,0,0-29.86,9.92A58.24,58.24,0,0,1,26.22,101.76ZM164,210H84a38,38,0,1,1,7.08-75.34,75.84,75.84,0,0,0-1.07,9,6,6,0,0,0,12,.7,61.54,61.54,0,0,1,2-12.24c0-.15.08-.29.11-.43A62.06,62.06,0,1,1,164,210Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M164,72a76.45,76.45,0,0,0-12.36,1A71.93,71.93,0,0,0,96.17,9.83a8,8,0,0,0-9.59,9.58A56.45,56.45,0,0,1,88,32,56.06,56.06,0,0,1,32,88a56.45,56.45,0,0,1-12.59-1.42,8,8,0,0,0-9.59,9.59,72.22,72.22,0,0,0,32.29,45.06A52,52,0,0,0,84,224h80a76,76,0,0,0,0-152ZM29.37,104c.87,0,1.75,0,2.63,0a72.08,72.08,0,0,0,72-72c0-.89,0-1.78,0-2.67a55.63,55.63,0,0,1,32,48,76.28,76.28,0,0,0-43,43.4A52,52,0,0,0,54,129.59,56.22,56.22,0,0,1,29.37,104ZM164,208H84a36,36,0,1,1,4.78-71.69c-.37,2.37-.63,4.79-.77,7.23a8,8,0,0,0,16,.92,58.91,58.91,0,0,1,1.88-11.81c0-.16.09-.32.12-.48A60.06,60.06,0,1,1,164,208Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M164,76A72,72,0,0,0,148,77.81,68,68,0,0,0,95.27,13.72a4,4,0,0,0-4.79,4.8,60.05,60.05,0,0,1-72,72,4,4,0,0,0-4.8,4.8A68.2,68.2,0,0,0,48.17,140.1,48,48,0,0,0,84,220h80a72,72,0,0,0,0-144ZM23.22,99.44A69.46,69.46,0,0,0,32,100a68.07,68.07,0,0,0,68-68,69.74,69.74,0,0,0-.56-8.79A59.66,59.66,0,0,1,140,80v.11a72.27,72.27,0,0,0-44.49,45.28A48.28,48.28,0,0,0,84,124a47.75,47.75,0,0,0-29.61,10.26A60.21,60.21,0,0,1,23.22,99.44ZM164,212H84a40,40,0,1,1,9.43-78.88A71.63,71.63,0,0,0,92,143.77a4,4,0,0,0,8,.46,64.3,64.3,0,0,1,2-12.67c0-.12.07-.24.09-.36A64.06,64.06,0,1,1,164,212Z"/> }.into_view()
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
