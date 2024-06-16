//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "finance", feature = "office"))]
#[component]
pub fn Invoice(
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
                <path d="M28,128a8,8,0,0,1,0-16H56a8,8,0,0,0,0-16H40a24,24,0,0,1,0-48,8,8,0,0,1,16,0h8a8,8,0,0,1,0,16H40a8,8,0,0,0,0,16H56a24,24,0,0,1,0,48,8,8,0,0,1-16,0ZM224,48H96a8,8,0,0,0,0,16H216V96H104a8,8,0,0,0,0,16h56v32H80a8,8,0,0,0,0,16h80v32H40V152a8,8,0,0,0-16,0v40a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A8,8,0,0,0,224,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,104v88a8,8,0,0,1-8,8H168V104Z" opacity="0.2"></path>
    <path d="M28,128a8,8,0,0,1,0-16H56a8,8,0,0,0,0-16H40a24,24,0,0,1,0-48,8,8,0,0,1,16,0h8a8,8,0,0,1,0,16H40a8,8,0,0,0,0,16H56a24,24,0,0,1,0,48,8,8,0,0,1-16,0ZM232,56V192a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V152a8,8,0,0,1,16,0v40H160V160H80a8,8,0,0,1,0-16h80V112H104a8,8,0,0,1,0-16H216V64H96a8,8,0,0,1,0-16H224A8,8,0,0,1,232,56Zm-56,88h40V112H176Zm40,48V160H176v32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M28,124a4,4,0,0,1,0-8H56a12,12,0,0,0,0-24H40a20,20,0,0,1,0-40h4V48a4,4,0,0,1,8,0v4H64a4,4,0,0,1,0,8H40a12,12,0,0,0,0,24H56a20,20,0,0,1,0,40H52v4a4,4,0,0,1-8,0v-4ZM228,56V192a12,12,0,0,1-12,12H40a12,12,0,0,1-12-12V152a4,4,0,0,1,8,0v40a4,4,0,0,0,4,4H164V156H80a4,4,0,0,1,0-8h84V108H104a4,4,0,0,1,0-8H220V60H96a4,4,0,0,1,0-8H224A4,4,0,0,1,228,56Zm-56,92h48V108H172Zm48,44V156H172v40h44A4,4,0,0,0,220,192Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M28,132a12,12,0,0,1,0-24H56a4,4,0,0,0,0-8H40a28,28,0,0,1-3.38-55.79A12,12,0,0,1,59.3,44H64a12,12,0,0,1,0,24H40a4,4,0,0,0,0,8H56a28,28,0,0,1,3.38,55.79A12,12,0,0,1,36.7,132ZM236,56V192a20,20,0,0,1-20,20H40a20,20,0,0,1-20-20V164a12,12,0,0,1,24,0v24H156V164H88a12,12,0,0,1,0-24h68V116H112a12,12,0,0,1,0-24H212V68H104a12,12,0,0,1,0-24H224A12,12,0,0,1,236,56Zm-56,84h32V116H180Zm32,48V164H180v24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M28,126a6,6,0,0,1,0-12H56a10,10,0,0,0,0-20H40a22,22,0,0,1,0-44h2V48a6,6,0,0,1,12,0v2H64a6,6,0,0,1,0,12H40a10,10,0,0,0,0,20H56a22,22,0,0,1,0,44H54v2a6,6,0,0,1-12,0v-2ZM230,56V192a14,14,0,0,1-14,14H40a14,14,0,0,1-14-14V152a6,6,0,0,1,12,0v40a2,2,0,0,0,2,2H162V158H80a6,6,0,0,1,0-12h82V110H104a6,6,0,0,1,0-12H218V62H96a6,6,0,0,1,0-12H224A6,6,0,0,1,230,56Zm-56,90h44V110H174Zm44,46V158H174v36h42A2,2,0,0,0,218,192Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M28,128a8,8,0,0,1,0-16H56a8,8,0,0,0,0-16H40a24,24,0,0,1,0-48,8,8,0,0,1,16,0h8a8,8,0,0,1,0,16H40a8,8,0,0,0,0,16H56a24,24,0,0,1,0,48,8,8,0,0,1-16,0ZM232,56V192a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V152a8,8,0,0,1,16,0v40H160V160H80a8,8,0,0,1,0-16h80V112H104a8,8,0,0,1,0-16H216V64H96a8,8,0,0,1,0-16H224A8,8,0,0,1,232,56Zm-56,88h40V112H176Zm40,48V160H176v32Z"></path>
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
