use web_sys::MouseEvent;
use yew::{Html, html, Properties, Children, Classes, classes, Callback, Component};

#[derive(Properties, PartialEq, Debug)]
pub struct OverlayProps {
    pub open: bool,

    #[prop_or_default]
    pub noshade: bool,

    #[prop_or_default]
    pub center: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<()>
}

#[derive(Debug)]
pub enum OverlayMessage {
    ClickContent,
    ClickRoot
}

pub struct Overlay {
    should_notify: bool,
    on_root_click: Callback<MouseEvent>,
    on_child_click: Callback<MouseEvent>,
}

impl Component for Overlay {
    type Message = OverlayMessage;
    type Properties = OverlayProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            should_notify: true,
            on_root_click: ctx.link().callback(|_| OverlayMessage::ClickRoot),
            on_child_click: ctx.link().callback(|_| OverlayMessage::ClickContent)
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let OverlayProps {
            open,
            noshade,
            center,
            children,
            class,
            onclick: _,
        } = ctx.props();
        html! {
            <div
                onclick={self.on_root_click.clone()}
                class={format!(
                    "overlay{}{}",
                    if *noshade { "" } else { " shade" },
                    if *open { "" } else { " hidden" }
                )}>
                <div
                    onclick={self.on_child_click.clone()}
                    class={classes!(if *center { "center" } else { "" }, class.clone())}>{children.clone()}</div>
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let mut notified = false;
        match msg {
            OverlayMessage::ClickContent => self.should_notify = false,
            OverlayMessage::ClickRoot => {
                if self.should_notify {
                    ctx.props().onclick.emit(());
                    notified = true;
                }

                self.should_notify = true;
            }
        }
        
        notified
    }

    fn changed(&mut self, ctx: &yew::Context<Self>, old_props: &Self::Properties) -> bool {
        ctx.props() != old_props
    }
}