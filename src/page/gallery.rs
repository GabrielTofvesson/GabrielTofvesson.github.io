use web_sys::MouseEvent;
use yew::{function_component, html, Html, Callback, use_state_eq};

use crate::component::image_viewer::{ImageDescription, ImageViewer};

const GALLERY: [&str; 13] = [
    "gallery-1.jpg",
    "gallery-2.jpg",
    "gallery-3.jpg",
    "gallery-4.jpg",
    "gallery-5.jpg",
    "gallery-6.jpg",
    "gallery-7.jpg",
    "gallery-8.jpg",
    "gallery-9.png",
    "gallery-10.jpg",
    "gallery-11.jpg",
    "gallery-12.jpg",
    "gallery-13.jpg",
];

fn gallery_entry(link: &&str, onclick: Callback<MouseEvent>) -> Html {
    html! {
        <div><img src={format!("/img/{link}")} onclick={onclick}/></div>
    }
}

#[function_component]
pub fn Gallery() -> Html {
    let images = GALLERY.iter().map(|it| ImageDescription::new(it.to_string(), it)).collect::<Vec<ImageDescription>>();
    let is_open = use_state_eq(|| false);
    let selected_image = use_state_eq(|| 0);
    let select_next = selected_image.setter();
    let select_prev = selected_image.setter();
    let next = (*selected_image + images.len() + 1) % images.len();
    let prev = (*selected_image + images.len() - 1) % images.len();
    
    html! {
        <>
            <div class="image-gallery">
                {
                    GALLERY.iter().enumerate().map(|(index, img_str)| {
                        let select = selected_image.clone();
                        let open = is_open.clone();
                        gallery_entry(img_str, Callback::from(move |_| {
                            select.set(index);
                            open.set(true);
                        }))
                    }).collect::<Html>()
                }
            </div>
            <ImageViewer
                image={images[*selected_image].clone()}
                open={*is_open && !images.is_empty()}
                onclose={Callback::from(move |_| is_open.set(false))}
                has_next=true
                has_prev=true
                onnext={Callback::from(move |_| select_next.set(next))}
                onprev={Callback::from(move |_| select_prev.set(prev))}
            />
        </>
    }
}