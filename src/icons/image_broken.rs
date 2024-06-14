//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn ImageBroken(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16h64a8,8,0,0,0,7.59-5.47l14.83-44.48L163,151.43a8.07,8.07,0,0,0,4.46-4.46l14.62-36.55,44.48-14.83A8,8,0,0,0,232,88V56A16,16,0,0,0,216,40ZM117,152.57a8,8,0,0,0-4.62,4.9L98.23,200H40V160.69l46.34-46.35a8,8,0,0,1,11.32,0l32.84,32.84Zm115-30.84V200a16,16,0,0,1-16,16H137.73a8,8,0,0,1-7.59-10.53l7.94-23.8a8,8,0,0,1,4.61-4.9l35.77-14.31,14.31-35.77a8,8,0,0,1,4.9-4.61l23.8-7.94A8,8,0,0,1,232,121.73Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,56V88l-48,16-16,40-23.35,9.34-39-39a8,8,0,0,0-11.32,0L32,168.69V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z"
        opacity="0.2"
    ></path>
    <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16h64a8,8,0,0,0,7.59-5.47l14.83-44.48L163,151.43a8.07,8.07,0,0,0,4.46-4.46l14.62-36.55,44.48-14.83A8,8,0,0,0,232,88V56A16,16,0,0,0,216,40ZM112.41,157.47,98.23,200H40V172l52-52,30.42,30.42L117,152.57A8,8,0,0,0,112.41,157.47ZM216,82.23,173.47,96.41a8,8,0,0,0-4.9,4.62l-14.72,36.82L138.58,144l-35.27-35.27a16,16,0,0,0-22.62,0L40,149.37V56H216Zm12.68,33a8,8,0,0,0-7.21-1.1l-23.8,7.94a8,8,0,0,0-4.9,4.61l-14.31,35.77-35.77,14.31a8,8,0,0,0-4.61,4.9l-7.94,23.8A8,8,0,0,0,137.73,216H216a16,16,0,0,0,16-16V121.73A8,8,0,0,0,228.68,115.24ZM216,200H148.83l3.25-9.75,35.51-14.2a8.07,8.07,0,0,0,4.46-4.46l14.2-35.51,9.75-3.25Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,44H40A12,12,0,0,0,28,56V200a12,12,0,0,0,12,12h64a4,4,0,0,0,3.79-2.74L123.21,163l38.28-15.31a4,4,0,0,0,2.22-2.22L179,107.21l46.24-15.42A4,4,0,0,0,228,88V56A12,12,0,0,0,216,44ZM118.51,156.29a4,4,0,0,0-2.3,2.45L101.12,204H40a4,4,0,0,1-4-4V170.34l53.17-53.17a4,4,0,0,1,5.66,0l34.71,34.71ZM220,85.12l-45.26,15.09a4,4,0,0,0-2.45,2.3l-15.37,38.41-19.3,7.73-37.13-37.14a12,12,0,0,0-17,0L36,159V56a4,4,0,0,1,4-4H216a4,4,0,0,1,4,4Zm6.34,33.37a4,4,0,0,0-3.6-.55l-23.81,7.93a4,4,0,0,0-2.44,2.31l-15,37.36-37.36,15a4,4,0,0,0-2.31,2.44l-7.93,23.81a4,4,0,0,0,.55,3.6,4,4,0,0,0,3.24,1.66H216a12,12,0,0,0,12-12V121.73A4,4,0,0,0,226.34,118.49ZM220,200a4,4,0,0,1-4,4H143.28l5.59-16.78,37.23-14.89a4,4,0,0,0,2.23-2.23l14.89-37.23L220,127.28Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M231,112a12,12,0,0,0-10.81-1.65l-23.81,7.93a12,12,0,0,0-7.34,6.93l-13.67,34.18-34.18,13.67a12,12,0,0,0-6.93,7.34l-7.93,23.81A12,12,0,0,0,137.73,220H216a20,20,0,0,0,20-20V121.73A12,12,0,0,0,231,112Zm-19,84H154.38l.91-2.73,33.79-13.51a12,12,0,0,0,6.68-6.68l13.51-33.79,2.73-.91Zm4-160H40A20,20,0,0,0,20,56V200a20,20,0,0,0,20,20H95.57A12,12,0,0,0,107,211.79L123.21,163l35.09-14A12,12,0,0,0,165,142.3l14-35.09L227.79,91A12,12,0,0,0,236,79.57V56A20,20,0,0,0,216,36ZM102.2,150.16,86.92,196H44V173.66l48-48,17.14,17.14h0A12,12,0,0,0,102.2,150.16ZM212,70.92,166.16,86.2a12,12,0,0,0-7.35,6.93l-14.2,35.48-11.22,4.49-27.25-27.24a20,20,0,0,0-28.28,0L44,139.72V60H212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,42H40A14,14,0,0,0,26,56V200a14,14,0,0,0,14,14h64a6,6,0,0,0,5.69-4.1l15.12-45.36,37.42-15a6,6,0,0,0,3.34-3.34l15-37.42L225.9,93.69A6,6,0,0,0,230,88V56A14,14,0,0,0,216,42ZM117.77,154.43a6,6,0,0,0-3.46,3.67L99.68,202H40a2,2,0,0,1-2-2V171.17l52.58-52.58a2,2,0,0,1,2.83,0L126,151.15ZM218,83.68,174.1,98.31a6,6,0,0,0-3.67,3.46l-15.05,37.61L138.1,146.3l-36.2-36.2a14,14,0,0,0-19.8,0L38,154.2V56a2,2,0,0,1,2-2H216a2,2,0,0,1,2,2Zm9.51,33.18a6,6,0,0,0-5.41-.82L198.3,124a6,6,0,0,0-3.67,3.47L180,164l-36.56,14.63A6,6,0,0,0,140,182.3L132,206.1a6,6,0,0,0,5.69,7.9H216a14,14,0,0,0,14-14V121.73A6,6,0,0,0,227.51,116.86ZM218,200a2,2,0,0,1-2,2H146.06l4.42-13.26,36.37-14.55a6,6,0,0,0,3.34-3.34l14.55-36.37L218,130.06Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16h64a8,8,0,0,0,7.59-5.47l14.83-44.48L163,151.43a8.07,8.07,0,0,0,4.46-4.46l14.62-36.55,44.48-14.83A8,8,0,0,0,232,88V56A16,16,0,0,0,216,40ZM112.41,157.47,98.23,200H40V172l52-52,30.42,30.42L117,152.57A8,8,0,0,0,112.41,157.47ZM216,82.23,173.47,96.41a8,8,0,0,0-4.9,4.62l-14.72,36.82L138.58,144l-35.27-35.27a16,16,0,0,0-22.62,0L40,149.37V56H216Zm12.68,33a8,8,0,0,0-7.21-1.1l-23.8,7.94a8,8,0,0,0-4.9,4.61l-14.31,35.77-35.77,14.31a8,8,0,0,0-4.61,4.9l-7.94,23.8A8,8,0,0,0,137.73,216H216a16,16,0,0,0,16-16V121.73A8,8,0,0,0,228.68,115.24ZM216,200H148.83l3.25-9.75,35.51-14.2a8.07,8.07,0,0,0,4.46-4.46l14.2-35.51,9.75-3.25Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().map(|id| id.get())
            class=move || class.get().map(|cls| cls.get())
        >
            {body}
        </svg>
    }
}
