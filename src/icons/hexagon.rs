//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design"))]
#[component]
pub fn Hexagon(
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
                <path d="M232,80.18v95.64a16,16,0,0,1-8.32,14l-88,48.17a15.88,15.88,0,0,1-15.36,0l-88-48.17a16,16,0,0,1-8.32-14V80.18a16,16,0,0,1,8.32-14l88-48.17a15.88,15.88,0,0,1,15.36,0l88,48.17A16,16,0,0,1,232,80.18Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,80.18v95.64a8,8,0,0,1-4.16,7l-88,48.18a8,8,0,0,1-7.68,0l-88-48.18a8,8,0,0,1-4.16-7V80.18a8,8,0,0,1,4.16-7l88-48.18a8,8,0,0,1,7.68,0l88,48.18A8,8,0,0,1,224,80.18Z"
        opacity="0.2"
    ></path>
    <path d="M223.68,66.15,135.68,18a15.88,15.88,0,0,0-15.36,0l-88,48.17a16,16,0,0,0-8.32,14v95.64a16,16,0,0,0,8.32,14l88,48.17a15.88,15.88,0,0,0,15.36,0l88-48.17a16,16,0,0,0,8.32-14V80.18A16,16,0,0,0,223.68,66.15ZM216,175.82,128,224,40,175.82V80.18L128,32h0l88,48.17Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M221.76,69.66l-88-48.18a12,12,0,0,0-11.52,0l-88,48.18A12,12,0,0,0,28,80.18v95.64a12,12,0,0,0,6.24,10.52l88,48.18a11.95,11.95,0,0,0,11.52,0l88-48.18A12,12,0,0,0,228,175.82V80.18A12,12,0,0,0,221.76,69.66ZM220,175.82a4,4,0,0,1-2.08,3.5l-88,48.18a4,4,0,0,1-3.84,0l-88-48.18a4,4,0,0,1-2.08-3.5V80.18a4,4,0,0,1,2.08-3.5l88-48.18a3.94,3.94,0,0,1,3.84,0l88,48.18a4,4,0,0,1,2.08,3.5Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M225.6,62.64l-88-48.17a19.91,19.91,0,0,0-19.2,0l-88,48.17A20,20,0,0,0,20,80.19v95.62a20,20,0,0,0,10.4,17.55l88,48.17a19.89,19.89,0,0,0,19.2,0l88-48.17A20,20,0,0,0,236,175.81V80.19A20,20,0,0,0,225.6,62.64ZM212,173.44l-84,46-84-46V82.56l84-46,84,46Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M222.72,67.91l-88-48.18a13.9,13.9,0,0,0-13.44,0l-88,48.18A14,14,0,0,0,26,80.18v95.64a14,14,0,0,0,7.28,12.27l88,48.18a13.92,13.92,0,0,0,13.44,0l88-48.18A14,14,0,0,0,230,175.82V80.18A14,14,0,0,0,222.72,67.91ZM218,175.82a2,2,0,0,1-1,1.75l-88,48.18a2,2,0,0,1-1.92,0L39,177.57a2,2,0,0,1-1-1.75V80.18a2,2,0,0,1,1-1.75l88-48.18a2,2,0,0,1,1.92,0l88,48.18a2,2,0,0,1,1,1.75Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M223.68,66.15,135.68,18h0a15.88,15.88,0,0,0-15.36,0l-88,48.17a16,16,0,0,0-8.32,14v95.64a16,16,0,0,0,8.32,14l88,48.17a15.88,15.88,0,0,0,15.36,0l88-48.17a16,16,0,0,0,8.32-14V80.18A16,16,0,0,0,223.68,66.15ZM216,175.82,128,224,40,175.82V80.18L128,32h0l88,48.17Z"></path>
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
