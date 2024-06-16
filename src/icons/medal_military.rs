//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects", feature = "games"))]
#[component]
pub fn MedalMilitary(
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
                <path d="M207,40H49A17,17,0,0,0,32,57v49.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,121.68a17,17,0,0,0,10-15.47V57A17,17,0,0,0,207,40ZM96,56h64v72.67l-32,14.54L96,128.67Zm32,168a32,32,0,1,1,32-32A32,32,0,0,1,128,224Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M168,192a40,40,0,1,1-40-40A40,40,0,0,1,168,192ZM207,48H168v85.82l42.72-19.42a9,9,0,0,0,5.28-8.2V57A9,9,0,0,0,207,48ZM88,48H49a9,9,0,0,0-9,9v49.2a9,9,0,0,0,5.28,8.2L88,133.82Z"
        opacity="0.2"
    ></path>
    <path d="M207,40H49A17,17,0,0,0,32,57v49.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,121.68a17,17,0,0,0,10-15.47V57A17,17,0,0,0,207,40ZM160,56v72.67l-32,14.54L96,128.67V56ZM48,106.21V57a1,1,0,0,1,1-1H80v65.39L48.59,107.12A1,1,0,0,1,48,106.21ZM128,224a32,32,0,1,1,32-32A32,32,0,0,1,128,224Zm80-117.79a1,1,0,0,1-.59.91L176,121.39V56h31a1,1,0,0,1,1,1Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M207,44H49A13,13,0,0,0,36,57v49.21A13,13,0,0,0,43.62,118l70.72,32.14a44,44,0,1,0,27.32,0L212.38,118A13,13,0,0,0,220,106.21V57A13,13,0,0,0,207,44Zm-43,8v79.24l-36,16.37L92,131.24V52ZM44,106.21V57a5,5,0,0,1,5-5H84v75.61L46.93,110.76A5,5,0,0,1,44,106.21ZM164,192a36,36,0,1,1-36-36A36,36,0,0,1,164,192Zm48-85.79a5,5,0,0,1-2.93,4.55L172,127.61V52h35a5,5,0,0,1,5,5Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M207,36H49A21,21,0,0,0,28,57v49.21a21,21,0,0,0,12.31,19.11l56,25.47a52,52,0,1,0,63.32,0l56-25.47A21,21,0,0,0,228,106.21V57A21,21,0,0,0,207,36ZM128,138.82l-28-12.73V60h56v66.09ZM52,60H76v55.18L52,104.27Zm76,160a28,28,0,1,1,28-28A28,28,0,0,1,128,220Zm76-115.73-24,10.91V60h24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M207,42H49A15,15,0,0,0,34,57v49.21a15,15,0,0,0,8.79,13.65L109.19,150a46,46,0,1,0,37.62,0l66.4-30.18A15,15,0,0,0,222,106.21V57A15,15,0,0,0,207,42ZM162,54v76l-34,15.45L94,130V54ZM46,106.21V57a3,3,0,0,1,3-3H82v70.5L47.76,108.94A3,3,0,0,1,46,106.21ZM162,192a34,34,0,1,1-34-34A34,34,0,0,1,162,192Zm48-85.79a3,3,0,0,1-1.76,2.73L174,124.5V54h33a3,3,0,0,1,3,3Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M207,40H49A17,17,0,0,0,32,57v49.21a17,17,0,0,0,10,15.47l62.6,28.45a48,48,0,1,0,46.88,0L214,121.68a17,17,0,0,0,10-15.47V57A17,17,0,0,0,207,40ZM160,56v72.67l-32,14.54L96,128.67V56ZM48,106.21V57a1,1,0,0,1,1-1H80v65.39L48.59,107.12A1,1,0,0,1,48,106.21ZM128,224a32,32,0,1,1,32-32A32,32,0,0,1,128,224Zm80-117.79a1,1,0,0,1-.59.91L176,121.39V56h31a1,1,0,0,1,1,1Z"></path>
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
