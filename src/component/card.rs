use web_sys::MouseEvent;
use yew::{Html, function_component, html, Children, Properties, Classes, Callback, classes};


#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: String,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html! {
        <div
            class={classes!("card", props.class.clone())}
            style={props.style.clone()}
            onclick={props.onclick.clone()}>
            {props.children.clone()}
        </div>
    }
}