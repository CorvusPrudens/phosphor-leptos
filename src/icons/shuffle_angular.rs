//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "arrows", feature = "system"))]
#[component]
pub fn ShuffleAngular(
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
                <path d="M237.66,178.34a8,8,0,0,1,0,11.32l-24,24A8,8,0,0,1,200,208V192H168a8,8,0,0,1-6.51-3.35L83.88,80H32a8,8,0,0,1,0-16H88a8,8,0,0,1,6.51,3.35L172.12,176H200V160a8,8,0,0,1,13.66-5.66ZM143,107a8,8,0,0,0,11.16-1.86l18-25.12H200V96a8,8,0,0,0,13.66,5.66l24-24a8,8,0,0,0,0-11.32l-24-24A8,8,0,0,0,200,48V64H168a8,8,0,0,0-6.51,3.35L141.15,95.82A8,8,0,0,0,143,107Zm-30,42a8,8,0,0,0-11.16,1.86L83.88,176H32a8,8,0,0,0,0,16H88a8,8,0,0,0,6.51-3.35l20.34-28.47A8,8,0,0,0,113,149Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M88,72l40,56L88,184H32V72Zm80,0-40,56,40,56h64V72Z" opacity="0.2"></path>
    <path d="M237.66,178.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,0,1-11.32-11.32L212.69,192H168a8,8,0,0,1-6.51-3.35L83.88,80H32a8,8,0,0,1,0-16H88a8,8,0,0,1,6.51,3.35L172.12,176h40.57l-10.35-10.34a8,8,0,0,1,11.32-11.32ZM143,107a8,8,0,0,0,11.16-1.86l18-25.12h40.57L202.34,90.34a8,8,0,0,0,11.32,11.32l24-24a8,8,0,0,0,0-11.32l-24-24a8,8,0,0,0-11.32,11.32L212.69,64H168a8,8,0,0,0-6.51,3.35L141.15,95.82A8,8,0,0,0,143,107Zm-30,42a8,8,0,0,0-11.16,1.86L83.88,176H32a8,8,0,0,0,0,16H88a8,8,0,0,0,6.51-3.35l20.34-28.47A8,8,0,0,0,113,149Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M234.83,181.17a4,4,0,0,1,0,5.66l-24,24a4,4,0,0,1-5.66-5.66L222.34,188H168a4,4,0,0,1-3.25-1.67L85.94,76H32a4,4,0,0,1,0-8H88a4,4,0,0,1,3.25,1.67L170.06,180h52.28l-17.17-17.17a4,4,0,0,1,5.66-5.66Zm-89.49-77.44a4,4,0,0,0,5.58-.93L170.06,76h52.28L205.17,93.17a4,4,0,0,0,5.66,5.66l24-24a4,4,0,0,0,0-5.66l-24-24a4,4,0,0,0-5.66,5.66L222.34,68H168a4,4,0,0,0-3.25,1.67L144.41,98.15A4,4,0,0,0,145.34,103.73Zm-34.68,48.54a4,4,0,0,0-5.58.93L85.94,180H32a4,4,0,0,0,0,8H88a4,4,0,0,0,3.25-1.67l20.34-28.48A4,4,0,0,0,110.66,152.27Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M240.49,175.51a12,12,0,0,1,0,17l-24,24a12,12,0,0,1-17-17L203,196H168a12,12,0,0,1-9.76-5L81.82,84H32a12,12,0,0,1,0-24H88a12,12,0,0,1,9.76,5l76.42,107H203l-3.52-3.51a12,12,0,0,1,17-17ZM110.4,152.64a12,12,0,0,0-16.74,2.79L81.82,172H32a12,12,0,0,0,0,24H88a12,12,0,0,0,9.76-5l15.43-21.59A12,12,0,0,0,110.4,152.64Zm35.2-49.28a12,12,0,0,0,16.74-2.79L174.18,84H203l-3.52,3.51a12,12,0,0,0,17,17l24-24a12,12,0,0,0,0-17l-24-24a12,12,0,0,0-17,17L203,60H168a12,12,0,0,0-9.76,5l-15.43,21.6A12,12,0,0,0,145.6,103.36Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M236.24,179.76a6,6,0,0,1,0,8.48l-24,24a6,6,0,0,1-8.48-8.48L217.52,190H168a6,6,0,0,1-4.88-2.51L84.91,78H32a6,6,0,0,1,0-12H88a6,6,0,0,1,4.88,2.51L171.09,178h46.43l-13.76-13.76a6,6,0,0,1,8.48-8.48Zm-92.07-74.4a6,6,0,0,0,8.37-1.4l18.55-26h46.43L203.76,91.76a6,6,0,1,0,8.48,8.48l24-24a6,6,0,0,0,0-8.48l-24-24a6,6,0,0,0-8.48,8.48L217.52,66H168a6,6,0,0,0-4.88,2.51L142.78,97A6,6,0,0,0,144.17,105.36Zm-32.34,45.28a6,6,0,0,0-8.37,1.4L84.91,178H32a6,6,0,0,0,0,12H88a6,6,0,0,0,4.88-2.51L113.22,159A6,6,0,0,0,111.83,150.64Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M237.66,178.34a8,8,0,0,1,0,11.32l-24,24a8,8,0,0,1-11.32-11.32L212.69,192H168a8,8,0,0,1-6.51-3.35L83.88,80H32a8,8,0,0,1,0-16H88a8,8,0,0,1,6.51,3.35L172.12,176h40.57l-10.35-10.34a8,8,0,0,1,11.32-11.32ZM143,107a8,8,0,0,0,11.16-1.86l18-25.12h40.57L202.34,90.34a8,8,0,0,0,11.32,11.32l24-24a8,8,0,0,0,0-11.32l-24-24a8,8,0,0,0-11.32,11.32L212.69,64H168a8,8,0,0,0-6.51,3.35L141.15,95.82A8,8,0,0,0,143,107Zm-30,42a8,8,0,0,0-11.16,1.86L83.88,176H32a8,8,0,0,0,0,16H88a8,8,0,0,0,6.51-3.35l20.34-28.47A8,8,0,0,0,113,149Z"></path>
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
