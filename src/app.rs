use yew::prelude::*;
use yewprint::Icon;
use crate::page::Page;
use crate::theme::ThemeContext;
use crate::theme::ThemeMsg;
use crate::theme::ThemeProvider;
use crate::component::actionbar::{Actionbar, ActionbarOption};


#[function_component]
pub fn AppRoot() -> Html {
    html! {
        <ThemeProvider>
            <ThemedApp />
        </ThemeProvider>
    }
}

#[function_component]
fn ThemedApp() -> Html {
    // Helper function for generating tabs
    fn page_option(current: &Page, page: Page, state: UseStateHandle<Page>) -> ActionbarOption {
        ActionbarOption::new(
            page.name(),
            match page {
                Page::Home => Icon::Home,
                Page::Projects => Icon::Code,
                Page::Socials => Icon::SocialMedia,
                Page::Contact => Icon::Envelope,
            },
            Callback::from(move |_| state.set(page)),
            *current == page
        )
    }

    let ctx = use_context::<ThemeContext>().expect("No theme context supplied");
    let page_state = use_state_eq(|| Page::Home);
    let current_page = *page_state;

    html! {
        <>
            <Actionbar>
                {vec![
                    page_option(&current_page, Page::Home, page_state.clone()),
                    page_option(&current_page, Page::Projects, page_state.clone()),
                    page_option(&current_page, Page::Socials, page_state.clone()),
                    page_option(&current_page, Page::Contact, page_state),
                    ActionbarOption::new_opt(
                        None,
                        Some(Icon::Flash),
                        Some(Callback::from(move |_| ctx.dispatch(ThemeMsg::Toggle))),
                        true,
                        false,
                        false
                    )
                ]}
            </Actionbar>
            <div class="main">
                {current_page.content()}
            </div>
        </>
    }
}