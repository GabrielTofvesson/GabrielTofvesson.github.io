use yew::{function_component, Properties, Html, html, classes};

#[derive(PartialEq, Default)]
pub enum TagColor {
    #[default]
    Blue,
    Red,
    Orange,
    Green
}

impl TagColor {
    fn as_class(&self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Green => "green",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub color: TagColor,
    pub text: String,
}

#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    html! {
        <span class={classes!("tag", props.color.as_class())}><span>{props.text.clone()}</span></span>
    }
}