use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yewprint::{Collapse, Icon, ButtonGroup};
use yewprint::Button;
use yewprint::Icon::{MenuClosed, MenuOpen};

#[derive(PartialEq, Clone)]
pub struct ActionbarOption {
    name: Option<String>,
    icon: Option<Icon>,
    onclick: Option<Callback<MouseEvent>>,
    center_content: bool,
    selected: bool,
    fill: bool,
}

#[derive(PartialEq, Clone)]
struct ActionbarOptionState {
    inner: ActionbarOption,
    callback: UseStateHandle<bool>
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
    pub fn new_opt(name: Option<String>, icon: Option<Icon>, onclick: Option<Callback<MouseEvent>>, center_content: bool, selected: bool, fill: bool) -> ActionbarOption {
        ActionbarOption { name, icon, onclick, center_content, selected, fill }
    }

    pub fn new(name: &str, icon: Icon, onclick: Callback<MouseEvent>, selected: bool) -> ActionbarOption {
        ActionbarOption::new_opt(Some(name.to_owned()), Some(icon), Some(onclick), false, selected, true)
    }
}

impl Into<VNode> for ActionbarOptionState {
    fn into(self) -> VNode {
        let classes = if self.inner.center_content { classes!() } else { classes!("ab-navbutton") };
        let onclick = self.inner.onclick.unwrap_or_default();
        html! {
            <Button
                fill={self.inner.fill}
                large=true
                class={classes}
                icon={self.inner.icon}
                onclick={Callback::from(move |e| { self.callback.set(false); onclick.emit(e) })}
                active={self.inner.selected}>
                {self.inner.name}
            </Button>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionbarProps {
    pub children: ChildrenRenderer<ActionbarOption>
}

#[function_component]
pub fn Actionbar(props: &ActionbarProps) -> Html {
    let expanded = use_state_eq(|| false);
    let is_expanded = *expanded;
    let children = props.children.iter().map(|opt| ActionbarOptionState { inner: opt, callback: expanded.clone() }).collect::<Vec<ActionbarOptionState>>();

    html! {
        <div class="actionbar">
            <div class="ab-portrait">
                <Button onclick={Callback::from(move |_| expanded.set(!is_expanded))} icon={if is_expanded { MenuOpen } else { MenuClosed }} large=true />
                <Collapse is_open={is_expanded}>
                    <ButtonGroup class="ab-portrait-content" vertical=true>
                        {children.clone()}
                    </ButtonGroup>
                </Collapse>
            </div>
            <ButtonGroup class="ab-landscape" fill=true>
                {children}
            </ButtonGroup>
        </div>
    }
}