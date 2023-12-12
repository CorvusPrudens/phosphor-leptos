//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BaseballCap(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M128,20A108.12,108.12,0,0,0,20,128v56a27.86,27.86,0,0,0,15.26,24.93,28,28,0,0,0,29.28-2.34C76.2,198.11,96.68,188,128,188s51.8,10.1,63.46,18.58A28,28,0,0,0,236,184V128A108.12,108.12,0,0,0,128,20Zm84,108v1.87a170,170,0,0,0-33.29-14.3,170.81,170.81,0,0,0-23.45-67A84.14,84.14,0,0,1,212,128Zm-58.46-18.12a174.42,174.42,0,0,0-51.08,0A150,150,0,0,1,128,50.94,150.07,150.07,0,0,1,153.54,109.88Zm-52.8-61.31a170.81,170.81,0,0,0-23.45,67A170,170,0,0,0,44,129.87V128A84.14,84.14,0,0,1,100.74,48.57Zm109.11,139a4,4,0,0,1-4.28-.36C191,176.61,165.77,164,128,164s-63,12.61-77.57,23.18a4,4,0,0,1-4.28.36A3.76,3.76,0,0,1,44,184V158.14a148,148,0,0,1,168,0V184A3.76,3.76,0,0,1,209.85,187.54Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M224,152v32a16,16,0,0,1-25.48,12.88C182.53,185.26,159,176,128,176s-54.53,9.26-70.52,20.88A16,16,0,0,1,32,184V152a160,160,0,0,1,192,0Z" opacity="0.2"/><path d="M128,24A104.12,104.12,0,0,0,24,128v56a24,24,0,0,0,24,24,24.11,24.11,0,0,0,14.18-4.64C74.33,194.53,95.6,184,128,184s53.67,10.52,65.81,19.35A24,24,0,0,0,232,184V128A104.12,104.12,0,0,0,128,24Zm88,104v8.87a166,166,0,0,0-40.94-18.22A167,167,0,0,0,146.19,41.9,88.15,88.15,0,0,1,216,128ZM128,44.27a152.47,152.47,0,0,1,30.4,70.46,170.85,170.85,0,0,0-60.84,0A153.31,153.31,0,0,1,128,44.27ZM109.81,41.9a167,167,0,0,0-28.87,76.76A166,166,0,0,0,40,136.88V128A88.15,88.15,0,0,1,109.81,41.9ZM211.66,191.11a8,8,0,0,1-8.44-.69C189.16,180.2,164.7,168,128,168S66.84,180.2,52.78,190.42a8,8,0,0,1-8.44.69A7.77,7.77,0,0,1,40,184V156.07a152,152,0,0,1,176,0V184A7.77,7.77,0,0,1,211.66,191.11Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M128,24A104.12,104.12,0,0,0,24,128v56a24,24,0,0,0,24,24,24.11,24.11,0,0,0,14.18-4.64C74.33,194.53,95.6,184,128,184s53.67,10.52,65.81,19.35A24,24,0,0,0,232,184V128A104.12,104.12,0,0,0,128,24ZM40,128A88.15,88.15,0,0,1,109.81,41.9a167,167,0,0,0-28.87,76.76A166,166,0,0,0,40,136.88Zm176,56a7.77,7.77,0,0,1-4.34,7.1,8,8,0,0,1-8.44-.69C189.16,180.2,164.7,168,128,168S66.84,180.2,52.78,190.42a8,8,0,0,1-8.44.69A7.77,7.77,0,0,1,40,184V156.07a150.62,150.62,0,0,1,49.93-23.28,7.06,7.06,0,0,0,1-.26,154.06,154.06,0,0,1,74.17,0,8.64,8.64,0,0,0,1,.27A150.49,150.49,0,0,1,216,156.07Zm0-47.13a166,166,0,0,0-40.94-18.22A167,167,0,0,0,146.19,41.9,88.15,88.15,0,0,1,216,128Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M128,26A102.12,102.12,0,0,0,26,128v56a22,22,0,0,0,35,17.74c12.38-9,34.06-19.74,67-19.74s54.61,10.73,67,19.73A22,22,0,0,0,230,184V128A102.12,102.12,0,0,0,128,26Zm90,102v12.5a164.29,164.29,0,0,0-44.8-20.3A165.07,165.07,0,0,0,141.69,39,90.15,90.15,0,0,1,218,128Zm-57.21-10.78a168.56,168.56,0,0,0-65.58,0c5-38.38,24.16-65.59,32.79-76.14C136.63,51.65,155.8,78.85,160.79,117.23ZM114.31,39A165.07,165.07,0,0,0,82.8,120.21,164.29,164.29,0,0,0,38,140.51V128A90.15,90.15,0,0,1,114.31,39Zm98.26,153.85A9.94,9.94,0,0,1,202,192c-13.82-10-37.88-22-74-22s-60.22,12-74,22a9.92,9.92,0,0,1-10.53.85A9.79,9.79,0,0,1,38,184V155a154,154,0,0,1,180,0v29A9.79,9.79,0,0,1,212.57,192.89Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M128,24h0A104.12,104.12,0,0,0,24,128v56a24,24,0,0,0,24,24,24.11,24.11,0,0,0,14.18-4.64C74.33,194.53,95.6,184,128,184s53.67,10.52,65.81,19.35A24,24,0,0,0,232,184V128A104.12,104.12,0,0,0,128,24Zm88,104v8.87a166,166,0,0,0-40.94-18.22A167,167,0,0,0,146.19,41.9,88.14,88.14,0,0,1,216,128ZM128,44.27a152.47,152.47,0,0,1,30.4,70.46,170.85,170.85,0,0,0-60.84,0A153.31,153.31,0,0,1,128,44.27ZM109.81,41.9a167,167,0,0,0-28.87,76.76A166,166,0,0,0,40,136.88V128A88.14,88.14,0,0,1,109.81,41.9ZM211.66,191.11a8,8,0,0,1-8.44-.69C189.16,180.2,164.7,168,128,168S66.84,180.2,52.78,190.42a8,8,0,0,1-8.44.69A7.77,7.77,0,0,1,40,184V156.07a152,152,0,0,1,176,0V184A7.77,7.77,0,0,1,211.66,191.11Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M128,28A100.12,100.12,0,0,0,28,128v56a20,20,0,0,0,31.84,16.12C72.46,191,94.53,180,128,180s55.54,10.94,68.16,20.12a20,20,0,0,0,21,1.67A19.87,19.87,0,0,0,228,184V128A100.12,100.12,0,0,0,128,28Zm92,100v16.23a162.31,162.31,0,0,0-48.67-22.47,162.56,162.56,0,0,0-34.15-85.31A92.12,92.12,0,0,1,220,128Zm-56.9-8.26a166.58,166.58,0,0,0-70.2,0C97.64,76.93,120,47.31,128,38,136,47.32,158.36,76.93,163.1,119.75ZM118.82,36.46a162.56,162.56,0,0,0-34.15,85.31A162.31,162.31,0,0,0,36,144.24V128A92.12,92.12,0,0,1,118.82,36.46Zm94.66,158.21a11.88,11.88,0,0,1-12.61-1C187.29,183.78,163.62,172,128,172s-59.29,11.77-72.87,21.65a11.88,11.88,0,0,1-12.61,1A11.75,11.75,0,0,1,36,184V154a156,156,0,0,1,184,0v30A11.75,11.75,0,0,1,213.48,194.67Z"/> }.into_view()
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
