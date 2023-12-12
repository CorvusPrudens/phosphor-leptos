//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Taxi(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M244,112h-9.45L204.46,65.18A19.93,19.93,0,0,0,187.63,56H168.12l-11-27.43A19.9,19.9,0,0,0,138.58,16H117.42A19.9,19.9,0,0,0,98.85,28.57L87.88,56H68.37a19.91,19.91,0,0,0-16.83,9.19L21.45,112H12a12,12,0,0,0,0,24h4v68a20,20,0,0,0,20,20H60a20,20,0,0,0,20-20V184h96v20a20,20,0,0,0,20,20h24a20,20,0,0,0,20-20V136h4a12,12,0,0,0,0-24ZM120.12,40h15.76l6.4,16H113.72ZM70.55,80h114.9L206,112H50ZM56,200H40V184H56Zm144,0V184h16v16Zm16-40H40V136H216Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M184,184h40v24a8,8,0,0,1-8,8H192a8,8,0,0,1-8-8ZM32,208a8,8,0,0,0,8,8H64a8,8,0,0,0,8-8V184H32ZM194.3,68a8,8,0,0,0-6.94-4H68.64a8,8,0,0,0-6.94,4L32,120H224Z" opacity="0.2"/><path d="M240,112H228.64L201.25,64.06A16,16,0,0,0,187.36,56H165.42l-12-29.94A15.93,15.93,0,0,0,138.58,16H117.42a15.93,15.93,0,0,0-14.86,10.06L90.58,56H68.64a16,16,0,0,0-13.89,8.06L27.36,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM117.42,32h21.16l9.6,24H107.82ZM68.64,72H187.36l22.85,40H45.79ZM64,208H40V192H64Zm128,0V192h24v16Zm24-32H40V128H216ZM56,152a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,152Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,152Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M240,112H228.64L201.25,64.06A16,16,0,0,0,187.36,56H165.42l-12-29.94A15.93,15.93,0,0,0,138.58,16H117.42a15.93,15.93,0,0,0-14.86,10.06L90.58,56H68.64a16,16,0,0,0-13.89,8.06L27.36,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM117.42,32h21.16l9.6,24H107.82ZM80,160H64a8,8,0,0,1,0-16H80a8,8,0,0,1,0,16Zm112,0H176a8,8,0,0,1,0-16h16a8,8,0,0,1,0,16Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M240,114H227.48l-28-48.95a14,14,0,0,0-12.15-7h-23.3L151.58,26.8a13.93,13.93,0,0,0-13-8.8H117.42a13.93,13.93,0,0,0-13,8.8L91.94,58H68.64a14,14,0,0,0-12.15,7l-28,49H16a6,6,0,0,0,0,12H26v82a14,14,0,0,0,14,14H64a14,14,0,0,0,14-14V190H178v18a14,14,0,0,0,14,14h24a14,14,0,0,0,14-14V126h10a6,6,0,0,0,0-12ZM115.56,31.26A2,2,0,0,1,117.42,30h21.16a2,2,0,0,1,1.86,1.26L151.14,58H104.86ZM66.91,71a2,2,0,0,1,1.73-1H187.36a2,2,0,0,1,1.73,1l24.57,43H42.34ZM66,208a2,2,0,0,1-2,2H40a2,2,0,0,1-2-2V190H66Zm150,2H192a2,2,0,0,1-2-2V190h28v18A2,2,0,0,1,216,210Zm2-32H38V126H218ZM58,152a6,6,0,0,1,6-6H80a6,6,0,0,1,0,12H64A6,6,0,0,1,58,152Zm112,0a6,6,0,0,1,6-6h16a6,6,0,0,1,0,12H176A6,6,0,0,1,170,152Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M240,112H228.64L201.25,64.06A16,16,0,0,0,187.36,56H165.42l-12-29.94A15.93,15.93,0,0,0,138.58,16H117.42a15.93,15.93,0,0,0-14.86,10.06L90.58,56H68.64a16,16,0,0,0-13.89,8.06L27.36,112H16a8,8,0,0,0,0,16h8v80a16,16,0,0,0,16,16H64a16,16,0,0,0,16-16V192h96v16a16,16,0,0,0,16,16h24a16,16,0,0,0,16-16V128h8a8,8,0,0,0,0-16ZM117.42,32h21.16l9.6,24H107.82ZM68.64,72H187.36l22.85,40H45.79ZM64,208H40V192H64Zm128,0V192h24v16Zm24-32H40V128H216ZM56,152a8,8,0,0,1,8-8H80a8,8,0,0,1,0,16H64A8,8,0,0,1,56,152Zm112,0a8,8,0,0,1,8-8h16a8,8,0,0,1,0,16H176A8,8,0,0,1,168,152Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M240,116H226.32L197.78,66.05a12,12,0,0,0-10.42-6H162.71l-13-32.46A12,12,0,0,0,138.58,20H117.42a12,12,0,0,0-11.15,7.54L93.29,60H68.64a12,12,0,0,0-10.42,6L29.68,116H16a4,4,0,0,0,0,8H28v84a12,12,0,0,0,12,12H64a12,12,0,0,0,12-12V188H180v20a12,12,0,0,0,12,12h24a12,12,0,0,0,12-12V124h12a4,4,0,0,0,0-8ZM113.7,30.51A4,4,0,0,1,117.42,28h21.16a4,4,0,0,1,3.72,2.51L154.09,60H101.91ZM65.17,70a4,4,0,0,1,3.47-2H187.36a4,4,0,0,1,3.47,2l26.28,46H38.89ZM68,208a4,4,0,0,1-4,4H40a4,4,0,0,1-4-4V188H68Zm148,4H192a4,4,0,0,1-4-4V188h32v20A4,4,0,0,1,216,212Zm4-32H36V124H220ZM60,152a4,4,0,0,1,4-4H80a4,4,0,0,1,0,8H64A4,4,0,0,1,60,152Zm112,0a4,4,0,0,1,4-4h16a4,4,0,0,1,0,8H176A4,4,0,0,1,172,152Z"/> }.into_view()
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
