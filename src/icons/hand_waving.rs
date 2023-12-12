//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn HandWaving(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M81.61,247.27a12,12,0,0,1-16.8,2.41A131.23,131.23,0,0,1,29.67,210a12,12,0,1,1,20.79-12,107.45,107.45,0,0,0,28.73,32.48A12,12,0,0,1,81.61,247.27ZM223.66,98A92,92,0,0,1,64.31,190l-38-65.82A32,32,0,0,1,45.46,77.33L45,76.46A32,32,0,0,1,81,29.55,31.7,31.7,0,0,1,90.62,34,32,32,0,0,1,143,38.31L155.52,60a32,32,0,0,1,50.14,6.84Zm-20.78,12-18-31.18a8,8,0,0,0-13.87,8h0l10,17.31a12,12,0,0,1-4.39,16.39,28,28,0,0,0-10.25,38.25,12,12,0,0,1-20.79,12A52.09,52.09,0,0,1,154.93,107L122.24,50.31a8,8,0,0,0-13.86,8l26,45a12,12,0,0,1-20.79,12l-34-58.89a8,8,0,0,0-10.92-2.93,8,8,0,0,0-2.93,10.93l38,65.81a12,12,0,1,1-20.79,12l-22-38.1a8,8,0,1,0-13.85,8L85.1,178a68,68,0,0,0,117.78-68ZM240.3,46.81a71.5,71.5,0,0,0-43.72-33.55,12,12,0,0,0-6.21,23.19,47.65,47.65,0,0,1,29.15,22.36,12,12,0,1,0,20.78-12Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M213.27,104l-18-31.18a20,20,0,0,0-34.64,20l-28-48.5A20,20,0,0,0,98,64.31L90,50.46a20,20,0,0,0-34.64,20l12,20.83-1.17,1A20,20,0,0,0,36.7,118.17L74.7,184a80,80,0,0,0,138.57-80Zm-57.59,60.64.17-.1.14.24Z" opacity="0.2"/><path d="M220.2,100l-18-31.18a28,28,0,0,0-47.3-1.92L139.56,40.31a28,28,0,0,0-48.12-.63,28,28,0,0,0-43,34.78l3.34,5.79a28,28,0,0,0-22,41.92l38,65.82a87.46,87.46,0,0,0,53.43,41,88.56,88.56,0,0,0,22.92,3A88,88,0,0,0,220.2,100Zm-6.67,62.63A72,72,0,0,1,81.63,180l-38-65.82a12,12,0,0,1,20.79-12l22,38.1a8,8,0,1,0,13.85-8l-38-65.81a12,12,0,0,1,13.5-17.59,11.9,11.9,0,0,1,7.29,5.59l34,58.89a8,8,0,0,0,13.85-8l-26-45h0a12,12,0,0,1,20.78-12L160,107.78a48.08,48.08,0,0,0-11,61,8,8,0,0,0,13.86-8,32,32,0,0,1,11.71-43.71,8,8,0,0,0,2.93-10.93l-10-17.32a12,12,0,0,1,20.78-12l18,31.18A71.49,71.49,0,0,1,213.53,162.62ZM184.27,29.93a8,8,0,0,1,9.8-5.66c15.91,4.27,29,14.11,36.86,27.73a8,8,0,0,1-13.86,8c-5.72-9.92-15.36-17.12-27.14-20.27A8,8,0,0,1,184.27,29.93ZM80.91,237a8,8,0,0,1-11.24,1.33c-11-8.69-20.11-19.58-28.6-34.28a8,8,0,0,1,13.86-8c7.44,12.88,15.27,22.32,24.65,29.72A8,8,0,0,1,80.91,237Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M184.27,29.93a8,8,0,0,1,9.8-5.66c15.9,4.27,29,14.11,36.86,27.73a8,8,0,0,1-13.86,8c-5.72-9.92-15.36-17.12-27.14-20.27A8,8,0,0,1,184.27,29.93ZM79.58,225.72c-9.38-7.4-17.22-16.84-24.65-29.72a8,8,0,0,0-13.86,8c8.49,14.7,17.57,25.59,28.6,34.28a8,8,0,0,0,9.91-12.56ZM196.19,58.42a16,16,0,1,0-27.71,16l16,27.71a8,8,0,0,1-2.93,10.93,32,32,0,0,0-11.71,43.71,8,8,0,0,1-13.86,8,48.07,48.07,0,0,1,11-61L126.63,33.92a16,16,0,0,0-27.72,16l32,55.43a8,8,0,0,1-13.85,8l-40-69.29a16,16,0,0,0-27.72,16l44,76.21a8,8,0,1,1-13.85,8l-28-48.49a16,16,0,1,0-27.72,16l44,76.21a88,88,0,0,0,152.42-88Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M186.2,30.45a6,6,0,0,1,7.35-4.24c15.39,4.12,28,13.64,35.65,26.79a6,6,0,0,1-10.4,6c-6-10.38-16.06-17.91-28.35-21.2A6,6,0,0,1,186.2,30.45ZM78.34,227.29C68.75,219.73,60.76,210.11,53.2,197a6,6,0,0,0-10.4,6c8.36,14.47,17.29,25.18,28.11,33.71a6,6,0,1,0,7.43-9.42Zm148.72-61A86.12,86.12,0,0,1,144.13,230a86.71,86.71,0,0,1-22.4-3A85.45,85.45,0,0,1,69.51,187l-38-65.82a26,26,0,0,1,23.68-39l-5-8.73A26,26,0,0,1,91.92,43a26,26,0,0,1,45.9-1.67l17.05,29.52a26,26,0,0,1,45.59-1l18,31.18A85.45,85.45,0,0,1,227.06,166.25Zm-19-59.26-18-31.18a14,14,0,1,0-24.25,14l10,17.32a6,6,0,0,1-2.19,8.2,34,34,0,0,0-12.45,46.44,6,6,0,1,1-10.39,6,46.08,46.08,0,0,1,11.84-59.5l-35.2-61a14,14,0,0,0-24.25,14l26,45a6,6,0,1,1-10.39,6l-34-58.89a14,14,0,1,0-24.25,14l38,65.81a6,6,0,1,1-10.39,6l-22-38.1a14,14,0,1,0-24.25,14L79.9,181a74,74,0,0,0,128.17-74Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M220.2,100l-18-31.18a28,28,0,0,0-47.3-1.92L139.56,40.31a28,28,0,0,0-48.12-.63,28,28,0,0,0-43,34.78l3.34,5.79a28,28,0,0,0-22,41.92l38,65.82a87.46,87.46,0,0,0,53.43,41,88.56,88.56,0,0,0,22.92,3A88,88,0,0,0,220.2,100Zm-6.67,62.63A72,72,0,0,1,81.63,180l-38-65.82a12,12,0,0,1,20.79-12l22,38.1a8,8,0,1,0,13.85-8l-38-65.81a12,12,0,0,1,13.5-17.59,11.9,11.9,0,0,1,7.29,5.59l34,58.89a8,8,0,0,0,13.85-8l-26-45h0a12,12,0,0,1,20.78-12L160,107.78a48.08,48.08,0,0,0-11,61,8,8,0,0,0,13.86-8,32,32,0,0,1,11.71-43.71,8,8,0,0,0,2.93-10.93l-10-17.32a12,12,0,0,1,20.78-12l18,31.18A71.49,71.49,0,0,1,213.53,162.62ZM184.27,29.93a8,8,0,0,1,9.8-5.66c15.91,4.27,29,14.11,36.86,27.73a8,8,0,0,1-13.86,8c-5.72-9.92-15.36-17.12-27.14-20.27A8,8,0,0,1,184.27,29.93ZM80.91,237a8,8,0,0,1-11.24,1.33c-11-8.69-20.11-19.58-28.6-34.28a8,8,0,0,1,13.86-8c7.44,12.88,15.27,22.32,24.65,29.72A8,8,0,0,1,80.91,237Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M188.14,31a4,4,0,0,1,4.9-2.83c14.87,4,27.1,13.17,34.42,25.86a4,4,0,1,1-6.92,4C214.28,47.16,203.77,39.3,191,35.86A4,4,0,0,1,188.14,31ZM77.1,228.86c-9.79-7.72-17.94-17.53-25.64-30.86a4,4,0,0,0-6.92,4c8.22,14.24,17,24.77,27.61,33.14a4,4,0,0,0,2.47.86,4,4,0,0,0,2.48-7.14Zm148-63.13a84.09,84.09,0,0,1-81,62.29,84.37,84.37,0,0,1-21.87-2.9,83.41,83.41,0,0,1-51-39.13l-38-65.82a24,24,0,0,1,25.7-35.49L51.88,72.46A24,24,0,0,1,92.48,47a24,24,0,0,1,43.61-4.64l19.08,33a24,24,0,0,1,43.56-4.54l18,31.18A83.43,83.43,0,0,1,225.12,165.73ZM209.8,106l-18-31.18a16,16,0,1,0-27.71,16l10,17.32a4,4,0,0,1-1.46,5.47,36,36,0,0,0-13.18,49.17,4,4,0,1,1-6.93,4,44.07,44.07,0,0,1,12.73-58L129.16,46.31a16,16,0,0,0-27.71,16l26,45a4,4,0,0,1-6.93,4l-34-58.89a16,16,0,1,0-27.71,16l38,65.81a4,4,0,1,1-6.93,4l-22-38.1a16,16,0,1,0-27.71,16l38,65.82A76,76,0,1,0,209.8,106Z"/> }.into_view()
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
