use yew::{function_component, Html, html, Callback, Properties, use_state};
use yewprint::{Overlay, Icon, IconSize};

const CHEVRON_SIZE: f64 = 40.0;

#[derive(PartialEq)]
pub struct ImageDescription {
    link: String,
    description: Option<&'static str>
}

impl ImageDescription {
    pub fn new(link: impl Into<String>, description: &'static str) -> Self {
        Self {
            link: link.into(),
            description: Some(description)
        }
    }

    pub fn new_blank(link: impl Into<String>) -> Self {
        Self {
            link: link.into(),
            description: None
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ImageViewerProps {
    pub images: Vec<ImageDescription>,
    pub open: bool,
    pub onclose: Callback<()>
}

#[function_component]
pub fn ImageViewer(props: &ImageViewerProps) -> Html {
    let selected_image = use_state(|| 0);
    let (select_next, select_prev) = (selected_image.clone(), selected_image.clone());
    html! {
        <Overlay
            class="gallery"
            open={props.open && !props.images.is_empty()}
            onclose={&props.onclose}>
            {
                if let Some(image_desc) = props.images.get(*selected_image) {
                    Some(html! {
                        <>
                            <img class="gallery-image" src={if image_desc.link.starts_with("https?://") { image_desc.link.to_string() } else { format!("/img/{}", image_desc.link) }} />
                            {
                                if let Some(description) = &image_desc.description {
                                    Some(html! {
                                        <div class="gallery-image-description">{description}</div>
                                    })
                                } else { None }
                            }

                            {
                                // Controls: Next
                                if *selected_image < props.images.len() - 1 {
                                    Some(html! {
                                        <Icon
                                            icon={yewprint::Icon::ChevronRight}
                                            intent={yewprint::Intent::Warning}
                                            size={IconSize(CHEVRON_SIZE)}
                                            onclick={Callback::from(move |_| select_next.set(*select_next + 1))}
                                            class="gallery-next"
                                            />
                                    })
                                } else { None }
                            }

                            {
                                // Controls: Prev
                                if *selected_image > 0 {
                                    Some(html! {
                                        <Icon
                                            icon={yewprint::Icon::ChevronLeft}
                                            intent={yewprint::Intent::Warning}
                                            size={IconSize(CHEVRON_SIZE)}
                                            onclick={Callback::from(move |_| select_prev.set(*select_prev - 1))}
                                            class="gallery-prev"
                                            />
                                    })
                                } else { None }
                            }
                        </>
                    })
                } else { None }
            }
        </Overlay>
    }
}