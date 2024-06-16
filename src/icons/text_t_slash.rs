//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor"))]
#[component]
pub fn TextTSlash(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM128,72h48a8,8,0,0,1,8,8V96a8,8,0,0,1-16,0V88H128a8,8,0,0,1,0-16Zm61.27,126a8,8,0,0,1-11.29-.75l-42-48V176h12a8,8,0,0,1,0,16H108a8,8,0,0,1,0-16h12V131L88,94.43V96a8,8,0,0,1-16,0V80a8.13,8.13,0,0,1,.63-3.13L66,69.27A8,8,0,0,1,78,58.73l112,128A8,8,0,0,1,189.27,198Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M200,56V200H72a16,16,0,0,1-16-16V56Z" opacity="0.2"></path>
    <path d="M213.38,221.92a8,8,0,0,1-11.3-.54L136,148.69V192h24a8,8,0,0,1,0,16H96a8,8,0,0,1,0-16h24V131.09L64,69.49V88a8,8,0,0,1-16,0V56a8,8,0,0,1,.72-3.31l-6.64-7.31A8,8,0,1,1,53.92,34.62l160,176A8,8,0,0,1,213.38,221.92ZM105.79,64H120V80.43a8,8,0,0,0,16,0V64h56V88a8,8,0,0,0,16,0V56a8,8,0,0,0-8-8H105.79a8,8,0,0,0,0,16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.69,219a4,4,0,0,1-5.65-.27l-73-80.34V196h28a4,4,0,0,1,0,8H96a4,4,0,0,1,0-8h28V129.55L60.78,60H60V88a4,4,0,0,1-8,0V56a4,4,0,0,1,2-3.45l-9-9.86A4,4,0,0,1,51,37.31l160,176A4,4,0,0,1,210.69,219ZM105.79,60H124V80.43a4,4,0,0,0,8,0V60h64V88a4,4,0,0,0,8,0V56a4,4,0,0,0-4-4H105.79a4,4,0,0,0,0,8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.07,224.88a12,12,0,0,1-16.95-.81L140,159v29h20a12,12,0,0,1,0,24H96a12,12,0,0,1,0-24h20V132.64L68,79.84V88a12,12,0,0,1-24,0V56a11.75,11.75,0,0,1,.23-2.31l-5.11-5.62A12,12,0,1,1,56.88,31.93l160,176A12,12,0,0,1,216.07,224.88ZM116,68v.57a12,12,0,1,0,24,0V68h48V88a12,12,0,0,0,24,0V56a12,12,0,0,0-12-12H116.6a12,12,0,0,0-.6,24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212,220.44a6,6,0,0,1-8.48-.4L134,143.52V194h26a6,6,0,0,1,0,12H96a6,6,0,0,1,0-12h26V130.32l-60-66V88a6,6,0,0,1-12,0V56a6,6,0,0,1,1.19-3.57L43.56,44A6,6,0,0,1,52.44,36l160,176A6,6,0,0,1,212,220.44ZM105.79,62H122V80.43a6,6,0,0,0,12,0V62h60V88a6,6,0,0,0,12,0V56a6,6,0,0,0-6-6H105.79a6,6,0,0,0,0,12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.38,221.92a8,8,0,0,1-11.3-.54L136,148.69V192h24a8,8,0,0,1,0,16H96a8,8,0,0,1,0-16h24V131.09L64,69.49V88a8,8,0,0,1-16,0V56a8,8,0,0,1,.72-3.31l-6.64-7.31A8,8,0,1,1,53.92,34.62l160,176A8,8,0,0,1,213.38,221.92ZM105.79,64H120V80.43a8,8,0,0,0,16,0V64h56V88a8,8,0,0,0,16,0V56a8,8,0,0,0-8-8H105.79a8,8,0,0,0,0,16Z"></path>
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
