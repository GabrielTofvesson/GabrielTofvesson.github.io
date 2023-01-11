use web_sys::MouseEvent;
use yew::{function_component, Html, html, Callback, Properties, use_state};
use yewprint::{Overlay, Icon, IconSize, Intent};

const CHEVRON_SIZE: f64 = 40.0;
const CHEVRON_TYPE: Intent = Intent::Danger;

#[derive(PartialEq, Clone)]
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
pub struct DefaultImageViewerProps {
    pub images: Vec<ImageDescription>,
    pub open: bool,
    #[prop_or_default]
    pub infinite: bool,
    pub onclose: Callback<()>,
}

#[function_component]
pub fn DefaultImageViewer(props: &DefaultImageViewerProps) -> Html {
    let selected_image = use_state(|| 0);
    let (select_next, select_prev) = (selected_image.clone(), selected_image.clone());

    let (has_next, has_prev) = (props.infinite || *selected_image < props.images.len() - 1, props.infinite || *selected_image > 0);
    let next = (*select_next + props.images.len() + 1) % props.images.len();
    let prev = (*select_prev + props.images.len() - 1) % props.images.len();

    html! {
        <ImageViewer
            image={props.images[*selected_image].clone()}
            open={props.open && !props.images.is_empty()}
            onclose={props.onclose.clone()}
            has_next={has_next}
            has_prev={has_prev}
            onnext={Callback::from(move |_| if has_next {
                select_next.set(next);
            })}
            onprev={Callback::from(move |_| if has_prev {
                select_prev.set(prev);
            })}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct ImageViewerProps {
    pub image: Option<ImageDescription>,
    pub open: bool,
    #[prop_or_default]
    pub has_prev: bool,
    #[prop_or_default]
    pub has_next: bool,
    pub onclose: Callback<()>,
    pub onnext: Callback<MouseEvent>,
    pub onprev: Callback<MouseEvent>,
}

#[function_component]
pub fn ImageViewer(props: &ImageViewerProps) -> Html {
    let onclose = props.onclose.clone();
    html! {
        <>
            <Overlay
                class="gallery"
                open={props.open && props.image.is_some()}
                onclose={&props.onclose}>
                <>
                    <div onclick={Callback::from(move |_| onclose.emit(()))}></div>
                    {
                        if let Some(image_desc) = props.image.as_ref() {
                            let onclose = props.onclose.clone();
                            Some(html! {
                                <>
                                    <div onclick={Callback::from(move |_| onclose.emit(()))}>
                                        <img src={if image_desc.link.starts_with("https?://") { image_desc.link.to_string() } else { format!("/img/{}", image_desc.link) }} />
                                        {
                                            if let Some(description) = &image_desc.description {
                                                Some(html! {
                                                    <div>{description}</div>
                                                })
                                            } else { None }
                                        }
                                    </div>

                                    {
                                        // Controls: Next
                                        if props.has_next {
                                            Some(html! {
                                                <Icon
                                                    icon={yewprint::Icon::ChevronRight}
                                                    intent={CHEVRON_TYPE}
                                                    size={IconSize(CHEVRON_SIZE)}
                                                    onclick={props.onnext.clone()}
                                                    class="gallery-next"
                                                    />
                                            })
                                        } else { None }
                                    }

                                    {
                                        // Controls: Prev
                                        if props.has_prev {
                                            Some(html! {
                                                <Icon
                                                    icon={yewprint::Icon::ChevronLeft}
                                                    intent={CHEVRON_TYPE}
                                                    size={IconSize(CHEVRON_SIZE)}
                                                    onclick={props.onprev.clone()}
                                                    class="gallery-prev"
                                                    />
                                            })
                                        } else { None }
                                    }
                                </>
                            })
                        } else { None }
                    }
                </>
            </Overlay>
        </>
    }
}