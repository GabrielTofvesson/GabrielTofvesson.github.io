use yew::prelude::*;

mod home;
mod projects;
mod contact;
mod socials;

#[derive(PartialEq, Copy, Clone)]
pub enum Page {
    Home,
    Projects,
    Contact,
    Socials
}

impl Page {
    pub fn name(&self) -> &'static str{
        match self {
            Page::Home => "Home",
            Page::Projects => "Projects",
            Page::Socials => "Social Media",
            Page::Contact => "Contact"
        }
    }

    pub fn content(&self) -> Html {
        match self {
            Page::Home => html!{<home::Home />},
            Page::Contact => html!{<contact::Contact />},
            Page::Socials => html!{<socials::Socials />},
            Page::Projects => html!{<projects::Projects />},
        }
    }
}