use yew::prelude::*;
use yew::virtual_dom::VNode;

use super::fa_icon::{FAIcon, FontawesomeIcon};

#[derive(PartialEq, Clone)]
pub struct ActionbarOption {
    name: Option<String>,
    icon: Option<FontawesomeIcon>,
    onclick: Option<Callback<MouseEvent>>,
    center_content: bool,
    selected: bool,
    fill: bool,
}

#[derive(PartialEq, Clone)]
struct ActionbarOptionState {
    inner: ActionbarOption,
    callback: UseStateSetter<bool>
}

impl Into<VNode> for ActionbarOption {
    fn into(self) -> VNode {
        /*
        let classes = if self.center_content { classes!() } else { classes!("ab-navbutton") };
        html! {
            <Button fill=true large=true class={classes} icon={self.icon} onclick={self.onclick.unwrap_or_default()} active={self.selected}>{self.name}</Button>
        }
        */
        panic!("Stateless actionbar option")
    }
}

impl ActionbarOption {
    pub fn new_opt(name: Option<String>, icon: Option<FontawesomeIcon>, onclick: Option<Callback<MouseEvent>>, center_content: bool, selected: bool, fill: bool) -> ActionbarOption {
        ActionbarOption { name, icon, onclick, center_content, selected, fill }
    }

    pub fn new(name: &str, icon: FontawesomeIcon, onclick: Callback<MouseEvent>, selected: bool) -> ActionbarOption {
        ActionbarOption::new_opt(Some(name.to_owned()), Some(icon), Some(onclick), false, selected, true)
    }
}

impl Into<VNode> for ActionbarOptionState {
    fn into(self) -> VNode {
        let onclick = self.inner.onclick.unwrap_or_default();
        html! {
            <div
                class={classes!(
                    if self.inner.selected { "current" } else { "" },
                    if self.inner.icon.is_some() && self.inner.name.is_some() { "" } else { "small" }
                )}
                onclick={Callback::from(move |e| {
                    self.callback.set(false);
                    onclick.emit(e)
                })}>
                <div>
                    {if let Some(icon) = self.inner.icon { html!{ <FAIcon icon={icon} /> } } else { html!{} }}
                    <span>{self.inner.name}</span>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionbarProps {
    #[prop_or_default]
    pub mobile_title: Option<String>,
    pub items: Vec<ActionbarOption>
}

#[function_component]
pub fn Actionbar(props: &ActionbarProps) -> Html {
    let expanded = use_state_eq(|| false);
    let expanded_setter = expanded.setter();
    let is_expanded = *expanded;

    html! {
        <div class="navbar">
            <div style={if props.mobile_title.is_some() { "gap: 5px;" } else { "" }} onclick={Callback::from(move |_| expanded.set(!*expanded))}>
                <FAIcon
                    icon={if is_expanded { FontawesomeIcon::ChevronUp } else { FontawesomeIcon::ChevronDown }} />
                {
                    if let Some(ref title) = props.mobile_title {
                        html! {
                            <span>{title}</span>
                        }
                    } else { html! {} }
                }
            </div>
            <div class={if is_expanded { "open" } else { "" }}></div>
            <div>
                {props.items.iter()
                    .map(move |opt| ActionbarOptionState { inner: (*opt).clone(), callback: expanded_setter.clone() })
                    .collect::<Html>()}
            </div>
        </div>
    }
}