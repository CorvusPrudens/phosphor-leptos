//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "office"))]
#[component]
pub fn Signature(
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
                <path d="M80.3,120.26A58.29,58.29,0,0,1,81,97.07C83.32,87,87.89,80,92.1,80c2.57,0,2.94.67,3.12,1,.88,1.61,4,10.93-12.63,46.52A28.87,28.87,0,0,1,80.3,120.26ZM232,56V200a16,16,0,0,1-16,16H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56ZM84,160c2-3.59,3.94-7.32,5.9-11.14,10.34-.32,22.21-7.57,35.47-21.68,5,9.69,11.38,15.25,18.87,16.55,8,1.38,16-2.38,23.94-11.2,6,5.53,16.15,11.47,31.8,11.47a8,8,0,0,0,0-16c-17.91,0-24.3-10.88-24.84-11.86a7.83,7.83,0,0,0-6.54-4.51,8,8,0,0,0-7.25,3.6c-6.78,10-11.87,13.16-14.39,12.73-4-.69-9.15-10-11.23-18a8,8,0,0,0-14-3c-8.88,10.94-16.3,17.79-22.13,21.66,15.8-35.65,13.27-48.59,9.6-55.3C107.35,69.84,102.59,64,92.1,64,79.66,64,69.68,75,65.41,93.46a75,75,0,0,0-.83,29.81c1.7,8.9,5.17,15.73,10.16,20.12-3,5.81-6.09,11.43-9,16.61H56a8,8,0,0,0,0,16h.44c-4.26,7.12-7.11,11.59-7.18,11.69a8,8,0,0,0,13.48,8.62c.36-.55,5.47-8.57,12.29-20.31H200a8,8,0,0,0,0-16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M67.41,142.56C35.71,129.52,45.41,32.07,78,32,111.82,32.05,92,90.26,67.41,142.56Z"
        opacity="0.2"
    ></path>
    <path d="M232,168H63.86c2.66-5.24,5.33-10.63,8-16.11,14.94,1.65,32.58-8.78,52.66-31.14,5,13.46,14.45,30.93,30.58,31.25,9.09.18,18.11-5.2,27.42-16.37C189.31,143.75,203.3,152,232,152a8,8,0,0,0,0-16c-30.43,0-39.43-10.45-40-16.11a7.67,7.67,0,0,0-5.46-7.75,8.14,8.14,0,0,0-9.25,3.49C165.23,134.17,158,136.06,155.38,136c-8.26-.16-16.66-19.52-19.54-33.42a8,8,0,0,0-14.09-3.37C101.54,124.55,88,133.08,79.57,135.29,88.06,116.42,94.4,99.85,98.46,85.9c6.82-23.44,7.32-39.83,1.51-50.1-3-5.38-9.34-11.8-22.06-11.8C61.85,24,49.18,39.18,43.14,65.65c-3.59,15.71-4.18,33.21-1.62,48s7.87,25.55,15.59,31.94c-3.73,7.72-7.53,15.26-11.23,22.41H24a8,8,0,0,0,0,16H37.41c-11.32,21-20.12,35.64-20.26,35.88a8,8,0,1,0,13.71,8.24c.15-.26,11.27-18.79,24.7-44.12H232a8,8,0,0,0,0-16ZM58.74,69.21C62.72,51.74,70.43,40,77.91,40c5.33,0,7.1,1.86,8.13,3.67,3,5.33,6.52,24.19-21.66,86.39C56.12,118.78,53.31,93,58.74,69.21Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M232,172H57.32c4-7.83,8.18-16.11,12.21-24.56,15.14,3.55,34.13-8.06,56.54-34.59,3.67,12.5,12.53,34.83,29.08,35.15h.35c8.71,0,17.64-6.25,27.16-19.06C187.17,137.14,199.31,148,232,148a4,4,0,0,0,0-8c-42.88,0-44-19.24-44-20a4,4,0,0,0-7.35-2.18C168.74,136.1,160.42,140,155.48,140h-.18c-13-.25-21.29-26.52-23.38-36.61a4,4,0,0,0-7-1.69c-20.94,26.27-39.06,39.55-51.8,38.2C97.31,87.5,105.19,53.16,96.49,37.77,94,33.31,88.7,28,77.91,28,64.09,28,52.54,42.41,47,66.54A122.7,122.7,0,0,0,45.46,113c2.64,15.29,8.39,26.05,16.69,31.35-4.58,9.6-9.3,19-13.84,27.68H24a4,4,0,0,0,0,8H44.1c-12.87,24.17-23.37,41.68-23.53,41.94a4,4,0,0,0,1.37,5.49A3.93,3.93,0,0,0,24,228a4,4,0,0,0,3.43-1.94c.16-.27,11.85-19.75,25.72-46.06H232a4,4,0,0,0,0-8ZM53.34,111.61a114.81,114.81,0,0,1,1.49-43.29C58.4,52.7,66.34,36,77.91,36c7.52,0,10.23,3.26,11.61,5.7C99,58.4,79.92,106,65.6,137,59.76,132.69,55.46,123.83,53.34,111.61Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232,164H70.38c2.2-4.42,4.41-8.94,6.59-13.52l.79-1.66c13.52-1.65,28.69-11.3,45.38-28.87,5.31,12,14,24.73,27.59,27.29,5.26,1,13.51.79,22.93-5.82a57.86,57.86,0,0,0,9-7.91C190.9,141.2,205.68,148,232,148a12,12,0,0,0,0-24c-31,0-35.42-10.49-36-12.38.49-5.37-2.75-9.76-7.88-11.63s-11.17.78-14.22,5.46c-10,15.32-16.62,18.62-18.75,18.21-4.88-.91-12.39-15.54-15.44-30.09a12,12,0,0,0-21.13-5c-10.3,12.92-19.16,21.79-26.47,27.56,21.19-52.94,17-72.22,11.37-82.25C98.48,24.91,89.41,20,78,20,59.92,20,45.36,36.55,39,64.26c-3.45,15.1-4,31.81-1.46,45.85,2.65,14.71,8.36,25.7,16.68,32.23C50.69,149.78,47,157.06,43.47,164H24a12,12,0,0,0,0,24h6.7c-9.7,17.67-16.85,29.6-17,29.81a12,12,0,1,0,20.56,12.38C34.91,229.15,45.23,212,58,188H232a12,12,0,0,0,0-24ZM62.43,69.6C65.77,55,72.45,44,78,44c3.71,0,4.29,1,4.64,1.64,1.43,2.53,6.55,17.07-17.44,72.78a44.64,44.64,0,0,1-4-12.56C59.23,94.89,59.68,81.67,62.43,69.6Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M232,170H60.6c3.34-6.54,6.73-13.35,10.06-20.29a26.64,26.64,0,0,0,4.3.35c14.23,0,31.09-11.12,50.33-33.22,1,3,2.2,6.25,3.68,9.56,6.89,15.45,15.68,23.39,26.14,23.6h.39c8.81,0,17.71-5.77,27-17.58C188.33,140.68,201.62,150,232,150a6,6,0,0,0,0-12c-39.69,0-41.88-16.61-42-18.06a5.83,5.83,0,0,0-4.15-5.8,6.09,6.09,0,0,0-6.88,2.59C166.81,135.4,158.9,138.06,155.34,138c-10.78-.21-19.11-23.68-21.46-35a6,6,0,0,0-10.57-2.53C104.64,123.86,88,137,76.18,137.92,100,85.84,107.23,52.7,98.23,36.78,95.44,31.86,89.66,26,77.91,26c-15,0-27,14.62-32.82,40.1C38.28,95.94,42.12,132,59.62,145c-4.15,8.64-8.4,17.08-12.52,25H24a6,6,0,0,0,0,12H40.76C28.65,204.58,19,220.66,18.86,220.91a6,6,0,1,0,10.28,6.18c.16-.26,11.56-19.27,25.21-45.09H232a6,6,0,0,0,0-12ZM56.79,68.77C61.05,50.08,69.34,38,77.91,38c6.4,0,8.64,2.51,9.87,4.69,4,7,6,28.15-22.88,91C54.59,123.39,50.73,95.3,56.79,68.77Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M232,168H63.86c2.66-5.24,5.33-10.63,8-16.11,15,1.65,32.58-8.78,52.66-31.14,5,13.46,14.45,30.93,30.58,31.25,9.06.18,18.11-5.2,27.42-16.37C189.31,143.75,203.3,152,232,152a8,8,0,0,0,0-16c-30.43,0-39.43-10.45-40-16.11a7.67,7.67,0,0,0-5.46-7.75,8.14,8.14,0,0,0-9.25,3.49c-12.07,18.54-19.38,20.43-21.92,20.37-8.26-.16-16.66-19.52-19.54-33.42a8,8,0,0,0-14.09-3.37C101.54,124.55,88,133.08,79.57,135.29,88.06,116.42,94.4,99.85,98.46,85.9c6.82-23.44,7.32-39.83,1.51-50.1-3-5.38-9.34-11.8-22.06-11.8C61.85,24,49.18,39.18,43.14,65.65c-3.59,15.71-4.18,33.21-1.62,48s7.87,25.55,15.59,31.94c-3.73,7.72-7.53,15.26-11.23,22.41H24a8,8,0,0,0,0,16H37.41c-11.32,21-20.12,35.64-20.26,35.88a8,8,0,1,0,13.71,8.24c.15-.26,11.27-18.79,24.7-44.12H232a8,8,0,0,0,0-16ZM58.74,69.21C62.72,51.74,70.43,40,77.91,40c5.33,0,7.1,1.86,8.13,3.67,3,5.33,6.52,24.19-21.66,86.39C56.12,118.78,53.31,93,58.74,69.21Z"></path>
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
