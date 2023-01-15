use web_sys::MouseEvent;
use yew::{function_component, Html, html, Callback, Properties, Component};

use crate::component::{overlay::Overlay, fa_icon::{FAIcon, FontawesomeIcon, FontawesomeSize}};

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

/*
#[function_component]
pub fn DefaultImageViewer(props: &DefaultImageViewerProps) -> Html {
    let selected_image = use_state_eq(|| 0);
    let select_next = selected_image.setter();
    let select_prev = selected_image.setter();

    let has_next = props.infinite || *selected_image < props.images.len() - 1;
    let has_prev = props.infinite || *selected_image > 0;
    let next = (*selected_image + props.images.len() + 1) % props.images.len();
    let prev = (*selected_image + props.images.len() - 1) % props.images.len();
    let onclose = props.onclose.clone();

    html! {
        <ImageViewer
            image={props.images[*selected_image].clone()}
            open={props.open && !props.images.is_empty()}
            onclose={Callback::from(move |_| onclose.emit(()))}
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
*/

pub enum DefaultImageViewerMessage {
    Next,
    Prev
}

pub struct DefaultImageViewer {
    current: usize,
}

impl Component for DefaultImageViewer {
    type Message = DefaultImageViewerMessage;

    type Properties = DefaultImageViewerProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { current: 0 }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let DefaultImageViewerProps {
            images,
            open,
            infinite,
            onclose,
        } = ctx.props();
        
        let has_next = *infinite || self.current < images.len() - 1;
        let has_prev = *infinite || self.current > 0;
        let onclose = onclose.clone();

        html! {
            <ImageViewer
                image={images[self.current].clone()}
                open={*open && !images.is_empty()}
                onclose={Callback::from(move |_| onclose.emit(()))}
                has_next={has_next}
                has_prev={has_prev}
                onnext={if has_next { ctx.link().callback(|_| DefaultImageViewerMessage::Next)} else { Default::default() }}
                onprev={if has_prev { ctx.link().callback(|_| DefaultImageViewerMessage::Prev)} else { Default::default() }}
            />
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let images = &ctx.props().images;

        self.current = match msg {
            DefaultImageViewerMessage::Next => {
                if self.current == images.len() - 1 { 0 } else { self.current + 1 }
            },
            DefaultImageViewerMessage::Prev => {
                if self.current == 0 { images.len() - 1 } else { self.current - 1 }
            }
        };

        images.len() > 1
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
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
                class="overlay-gallery"
                open={props.open && props.image.is_some()}
                onclick={Callback::from(move |_| onclose.emit(()))}>
                <>
                    <div></div>
                    {
                        if let Some(image_desc) = props.image.as_ref() {
                            let onclose = props.onclose.clone();
                            Some(html! {
                                <>
                                    <div>
                                        <img
                                            onclick={Callback::from(move |_| onclose.emit(()))}
                                            src={
                                                if image_desc.link.starts_with("https?://") {
                                                    image_desc.link.to_string()
                                                } else {
                                                    format!("/img/{}", image_desc.link)
                                                }
                                            }
                                        />
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
                                                <div class="overlay-gallery-next">
                                                    <FAIcon
                                                        icon={FontawesomeIcon::ChevronRight}
                                                        size={FontawesomeSize::XL}
                                                        onclick={props.onnext.clone()}
                                                        />
                                                </div>
                                            })
                                        } else { None }
                                    }

                                    {
                                        // Controls: Prev
                                        if props.has_prev {
                                            Some(html! {
                                                <div class="overlay-gallery-prev">
                                                    <FAIcon
                                                        icon={FontawesomeIcon::ChevronLeft}
                                                        size={FontawesomeSize::XL}
                                                        onclick={props.onprev.clone()}
                                                        />
                                                </div>
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