use std::rc::Rc;

use gloo::utils::document;
use yew::{Children, Properties, Html, html};
use yew::prelude::*;
use crate::util::log;

fn set_global_dark(is_dark: bool) {
    let root = document().get_elements_by_tag_name("html").get_with_index(0).expect("Root html tag");
    let class_list = root.get_elements_by_tag_name("body").get_with_index(0).expect("Body tag").class_list();

    if is_dark {
        if let Err(_) = root.set_attribute("data-theme", "dark") {
            log("Couldn't set attribute 'data-theme' on root");
        }

        if !class_list.contains("bp3-dark") {
            if let Err(_) = class_list.add_1("bp3-dark") {
                log("Couldn't set class 'bp3-dark' on root")
            }
        }
    } else {
        if let Err(_) = root.remove_attribute("data-theme") {
            log("Couldn't remove attribute 'data-theme' from root");
        }

        if class_list.contains("bp3-dark") {
            if let Err(_) = class_list.remove_1("bp3-dark") {
                log("Couldn't remove class 'bp3-dark' from root")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeCtx {
    pub theme: ThemeState
}

impl Reducible for ThemeCtx {
    type Action = ThemeMsg;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let ctx: Rc<ThemeCtx> = ThemeCtx {
            theme: match action {
                //ThemeMsg::Dark => ThemeState::Dark,
                //ThemeMsg::Light => ThemeState::Light,
                ThemeMsg::Toggle => match self.theme {
                    ThemeState::Dark => ThemeState::Light,
                    ThemeState::Light => ThemeState::Dark
                }
            }
        }.into();

        set_global_dark(
            match &ctx.theme {
                ThemeState::Dark => true,
                ThemeState::Light => false
            }
        );

        ctx
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ThemeState {
    Dark, Light
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ThemeMsg {
    //Dark,
    //Light,
    Toggle
}

#[derive(Properties, PartialEq, Debug)]
pub struct ThemeProps {
    #[prop_or_default]
    pub children: Children
}

pub type ThemeContext = UseReducerHandle<ThemeCtx>;

#[function_component]
pub fn ThemeProvider(props: &ThemeProps) -> Html {
    let theme = use_reducer(|| {
        let dark_mode = web_sys::window()
            .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
            .map(|x| x.matches())
            .unwrap_or(true);
        set_global_dark(dark_mode);
        ThemeCtx {
            theme: if dark_mode {
                ThemeState::Dark
            } else {
                ThemeState::Light
            }
        }
    });

    html! {
        <ContextProvider<ThemeContext> context={theme}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}