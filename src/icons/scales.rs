//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Scales(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view!{ <path d="M243.14,131.54l-32-80h0a12,12,0,0,0-13.73-7.25L140,57V40a12,12,0,0,0-24,0V62.37L53.4,76.29a12,12,0,0,0-8.54,7.25h0l0,0v0l-32,79.92A12,12,0,0,0,12,168c0,12.13,6.2,22.43,17.45,29A55,55,0,0,0,56,204a55,55,0,0,0,26.55-7C93.8,190.43,100,180.13,100,168a12,12,0,0,0-.86-4.46L72.38,96.65,116,87V204H104a12,12,0,0,0,0,24h48a12,12,0,0,0,0-24H140V81.63l40.42-9-23.56,58.9A12,12,0,0,0,156,136c0,12.13,6.2,22.43,17.45,29a53.78,53.78,0,0,0,53.1,0C237.8,158.43,244,148.13,244,136A12,12,0,0,0,243.14,131.54ZM56,180c-3.71,0-18-1.87-19.81-10.18L56,120.31l19.81,49.51C74,178.13,59.71,180,56,180Zm144-32c-3.71,0-18-1.87-19.81-10.18L200,88.31l19.81,49.51C218,146.13,203.71,148,200,148Z"/> }.into_view(),
IconWeight::Duotone => view!{ <path d="M56,88l32,80c0,17.67-20,24-32,24s-32-6.33-32-24ZM200,56l-32,80c0,17.67,20,24,32,24s32-6.33,32-24Z" opacity="0.2"/><path d="M239.43,133l-32-80h0a8,8,0,0,0-9.16-4.84L136,62V40a8,8,0,0,0-16,0V65.58L54.26,80.19A8,8,0,0,0,48.57,85h0v.06L16.57,165a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32a7.92,7.92,0,0,0-.57-3L66.92,93.77,120,82V208H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16H136V78.42L187,67.1,160.57,133a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32A7.92,7.92,0,0,0,239.43,133ZM56,184c-7.53,0-22.76-3.61-23.93-14.64L56,109.54l23.93,59.82C78.76,180.39,63.53,184,56,184Zm144-32c-7.53,0-22.76-3.61-23.93-14.64L200,77.54l23.93,59.82C222.76,148.39,207.53,152,200,152Z"/> }.into_view(),
IconWeight::Fill => view!{ <path d="M239.43,133l-32-80A8,8,0,0,0,200,48a8.27,8.27,0,0,0-1.73.21L136,62V40a8,8,0,0,0-16,0V65.58L54.27,80.21A8,8,0,0,0,48.57,85l-32,80a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32a7.92,7.92,0,0,0-.57-3L66.92,93.77,120,82V208H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16H136V78.42L187,67.1,160.57,133a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32A7.92,7.92,0,0,0,239.43,133Zm-160,35H32.62L56,109.54Zm97.24-32L200,77.54,223.38,136Z"/> }.into_view(),
IconWeight::Light => view!{ <path d="M237.57,133.77l-32-80h0a6,6,0,0,0-6.86-3.63L134,64.52V40a6,6,0,0,0-12,0V67.19l-67.3,15a6,6,0,0,0-4.27,3.63h0v0l-32,80A6.1,6.1,0,0,0,18,168c0,21.86,23.31,30,38,30s38-8.14,38-30a6.1,6.1,0,0,0-.43-2.23L64.19,92.33,122,79.48V210H104a6,6,0,0,0,0,12h48a6,6,0,0,0,0-12H134V76.81l56.21-12.49-27.78,69.45A6.1,6.1,0,0,0,162,136c0,21.86,23.31,30,38,30s38-8.14,38-30A6.1,6.1,0,0,0,237.57,133.77ZM56,186a36.89,36.89,0,0,1-17.48-4.56c-5.37-3.13-8.15-7.18-8.49-12.37l26-64.91,26,64.91C81.06,182.85,62.58,186,56,186Zm144-32a36.89,36.89,0,0,1-17.48-4.56c-5.37-3.13-8.15-7.18-8.49-12.37l26-64.91,26,64.91C225.06,150.85,206.58,154,200,154Z"/> }.into_view(),
IconWeight::Regular => view!{ <path d="M239.43,133l-32-80h0a8,8,0,0,0-9.16-4.84L136,62V40a8,8,0,0,0-16,0V65.58L54.26,80.19A8,8,0,0,0,48.57,85h0v.06L16.57,165a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32a7.92,7.92,0,0,0-.57-3L66.92,93.77,120,82V208H104a8,8,0,0,0,0,16h48a8,8,0,0,0,0-16H136V78.42L187,67.1,160.57,133a7.92,7.92,0,0,0-.57,3c0,23.31,24.54,32,40,32s40-8.69,40-32A7.92,7.92,0,0,0,239.43,133ZM56,184c-7.53,0-22.76-3.61-23.93-14.64L56,109.54l23.93,59.82C78.76,180.39,63.53,184,56,184Zm144-32c-7.53,0-22.76-3.61-23.93-14.64L200,77.54l23.93,59.82C222.76,148.39,207.53,152,200,152Z"/> }.into_view(),
IconWeight::Thin => view!{ <path d="M235.71,134.51l-32-80h0a4,4,0,0,0-4.57-2.41L132,67V40a4,4,0,0,0-8,0V68.79L55.13,84.1a4,4,0,0,0-2.84,2.41h0v0h0l-32,80A4,4,0,0,0,20,168c0,20.4,22.08,28,36,28s36-7.6,36-28a4,4,0,0,0-.29-1.49L61.46,90.88,124,77V212H104a4,4,0,0,0,0,8h48a4,4,0,0,0,0-8H132V75.21l61.47-13.66-29.18,73A4,4,0,0,0,164,136c0,20.4,22.08,28,36,28s36-7.6,36-28A4,4,0,0,0,235.71,134.51ZM56,188c-7.15,0-27.37-3.56-28-19.27l28-70,28,70C83.37,184.44,63.15,188,56,188Zm144-32c-7.15,0-27.37-3.56-28-19.27l28-70,28,70C227.37,152.44,207.15,156,200,156Z"/> }.into_view()
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
