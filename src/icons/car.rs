//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Car(
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
                <path d="M240,104H229.2L201.42,41.5A16,16,0,0,0,186.8,32H69.2a16,16,0,0,0-14.62,9.5L26.8,104H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16v-8h96v8a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V120h8a8,8,0,0,0,0-16ZM80,152H56a8,8,0,0,1,0-16H80a8,8,0,0,1,0,16Zm120,0H176a8,8,0,0,1,0-16h24a8,8,0,0,1,0,16ZM44.31,104,69.2,48H186.8l24.89,56Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M184,176h40v24a8,8,0,0,1-8,8H192a8,8,0,0,1-8-8ZM32,200a8,8,0,0,0,8,8H64a8,8,0,0,0,8-8V176H32ZM194.11,44.75A8,8,0,0,0,186.8,40H69.2a8,8,0,0,0-7.31,4.75L32,112H224Z"
        opacity="0.2"
    ></path>
    <path d="M240,104H229.2L201.42,41.5A16,16,0,0,0,186.8,32H69.2a16,16,0,0,0-14.62,9.5L26.8,104H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V120h8a8,8,0,0,0,0-16ZM69.2,48H186.8l24.89,56H44.31ZM64,200H40V184H64Zm128,0V184h24v16Zm24-32H40V120H216ZM56,144a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,144Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,144Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M240,108H226.6L197.77,43.13a12,12,0,0,0-11-7.13H69.2a12,12,0,0,0-11,7.13L29.4,108H16a4,4,0,0,0,0,8H28v84a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V180H180v20a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V116h12a4,4,0,0,0,0-8ZM65.54,46.38A4,4,0,0,1,69.2,44H186.8a4,4,0,0,1,3.66,2.38L217.84,108H38.16ZM68,200a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V180H68Zm148,4H192a4,4,0,0,1-4-4V180h32v20A4,4,0,0,1,216,204Zm4-32H36V116H220ZM60,144a4,4,0,0,1,4-4H80a4,4,0,0,1,0,8H64A4,4,0,0,1,60,144Zm112,0a4,4,0,0,1,4-4h16a4,4,0,0,1,0,8H176A4,4,0,0,1,172,144Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240,100h-8.2L205.08,39.88A20,20,0,0,0,186.8,28H69.2A20,20,0,0,0,50.92,39.88L24.2,100H16a12,12,0,0,0,0,24h4v76a20,20,0,0,0,20,20H68a20,20,0,0,0,20-20V180h80v20a20,20,0,0,0,20,20h28a20,20,0,0,0,20-20V124h4a12,12,0,0,0,0-24ZM71.8,52H184.2l21.33,48H50.47ZM64,196H44V180H64Zm128,0V180h20v16Zm20-40H44V124H212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M240,106H227.9L199.59,42.31A14,14,0,0,0,186.8,34H69.2a14,14,0,0,0-12.79,8.31L28.1,106H16a6,6,0,0,0,0,12H26v82a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V182H178v18a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V118h10a6,6,0,0,0,0-12ZM67.37,47.19A2,2,0,0,1,69.2,46H186.8a2,2,0,0,1,1.83,1.19L214.77,106H41.23ZM66,200a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V182H66Zm150,2H192a2,2,0,0,1-2-2V182h28v18A2,2,0,0,1,216,202Zm2-32H38V118H218ZM58,144a6,6,0,0,1,6-6H80a6,6,0,0,1,0,12H64A6,6,0,0,1,58,144Zm112,0a6,6,0,0,1,6-6h16a6,6,0,0,1,0,12H176A6,6,0,0,1,170,144Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M240,104H229.2L201.42,41.5A16,16,0,0,0,186.8,32H69.2a16,16,0,0,0-14.62,9.5L26.8,104H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V184h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V120h8a8,8,0,0,0,0-16ZM69.2,48H186.8l24.89,56H44.31ZM64,200H40V184H64Zm128,0V184h24v16Zm24-32H40V120H216ZM56,144a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,144Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,144Z"></path>
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
