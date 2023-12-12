//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Car(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M240,108h-8.2L205.08,47.88A20,20,0,0,0,186.8,36H69.2A20,20,0,0,0,50.92,47.88L24.2,108H16a12,12,0,0,0,0,24h4v76a20,20,0,0,0,20,20H64a20,20,0,0,0,20-20V188h88v20a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20V132h4a12,12,0,0,0,0-24ZM71.8,60H184.2l21.33,48H50.47ZM60,204H44V188H60Zm136,0V188h16v16Zm16-40H44V132H212Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M184,184h40v24a8,8,0,0,1-8,8H192a8,8,0,0,1-8-8ZM32,208a8,8,0,0,0,8,8H64a8,8,0,0,0,8-8V184H32ZM194.11,52.75A8,8,0,0,0,186.8,48H69.2a8,8,0,0,0-7.31,4.75L32,120H224Z" opacity="0.2"/><path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM69.2,56H186.8l24.89,56H44.31ZM64,208H40V192H64Zm128,0V192h24v16Zm24-32H40V128H216ZM56,152a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,152Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,152Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM80,160H64a8,8,0,0,1,0-16H80a8,8,0,0,1,0,16Zm96,0a8,8,0,0,1,0-16h16a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M240,114H227.9L199.59,50.31A14,14,0,0,0,186.8,42H69.2a14,14,0,0,0-12.79,8.31L28.1,114H16a6,6,0,0,0,0,12H26v82a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V190H178v18a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V126h10a6,6,0,0,0,0-12ZM67.37,55.19A2,2,0,0,1,69.2,54H186.8a2,2,0,0,1,1.83,1.19L214.77,114H41.23ZM66,208a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V190H66Zm150,2H192a2,2,0,0,1-2-2V190h28v18A2,2,0,0,1,216,210Zm2-32H38V126H218ZM58,152a6,6,0,0,1,6-6H80a6,6,0,0,1,0,12H64A6,6,0,0,1,58,152Zm112,0a6,6,0,0,1,6-6h16a6,6,0,0,1,0,12H176A6,6,0,0,1,170,152Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,112H229.2L201.42,49.5A16,16,0,0,0,186.8,40H69.2a16,16,0,0,0-14.62,9.5L26.8,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM69.2,56H186.8l24.89,56H44.31ZM64,208H40V192H64Zm128,0V192h24v16Zm24-32H40V128H216ZM56,152a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,152Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,152Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M240,116H226.6L197.77,51.13a12,12,0,0,0-11-7.13H69.2a12,12,0,0,0-11,7.13L29.4,116H16a4,4,0,0,0,0,8H28v84a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V188H180v20a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V124h12a4,4,0,0,0,0-8ZM65.54,54.38A4,4,0,0,1,69.2,52H186.8a4,4,0,0,1,3.66,2.38L217.84,116H38.16ZM68,208a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V188H68Zm148,4H192a4,4,0,0,1-4-4V188h32v20A4,4,0,0,1,216,212Zm4-32H36V124H220ZM60,152a4,4,0,0,1,4-4H80a4,4,0,0,1,0,8H64A4,4,0,0,1,60,152Zm112,0a4,4,0,0,1,4-4h16a4,4,0,0,1,0,8H176A4,4,0,0,1,172,152Z"/> }.into_view()
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
