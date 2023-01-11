use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod projects;
mod contact;
mod socials;
mod gallery;

#[derive(PartialEq, Copy, Clone, Routable)]
pub enum Page {
    #[at("/")]
    Home,

    #[at("/projects")]
    Projects,

    #[at("/socials")]
    Socials,

    #[at("/photos")]
    Gallery,

    #[at("/contacts")]
    Contact,
}

fn switch(page: Page) -> Html {
    match page {
        Page::Home => html!{<home::Home />},
        Page::Projects => html!{<projects::Projects />},
        Page::Socials => html!{<socials::Socials />},
        Page::Gallery => html!{<gallery::Gallery />},
        Page::Contact => html!{<contact::Contact />},
    }
}

#[derive(Properties, PartialEq)]
pub struct PageRouterProps {
    pub children: Children
}

#[function_component]
pub fn PageRouter(props: &PageRouterProps) -> Html {
    html! {
        <BrowserRouter>
            {props.children.clone()}
        </BrowserRouter>
    }
}

#[function_component]
pub fn Pages() -> Html {
    html! {
        <Switch<Page> render={switch} />
    }
}

impl Page {
    pub fn name(&self) -> &'static str{
        match self {
            Page::Home => "Home",
            Page::Projects => "Projects",
            Page::Socials => "Social Media",
            Page::Gallery => "Gallery",
            Page::Contact => "Contact"
        }
    }
}