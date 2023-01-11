use gloo::utils::window;
use yew::prelude::*;
use yew_router::Routable;
use yew_router::prelude::Navigator;
use yew_router::prelude::use_navigator;
use yewprint::Icon;
use crate::page::Page;
use crate::page::PageRouter;
use crate::page::Pages;
use crate::theme::ThemeContext;
use crate::theme::ThemeMsg;
use crate::theme::ThemeProvider;
use crate::component::actionbar::{Actionbar, ActionbarOption};


#[function_component]
pub fn AppRoot() -> Html {
    html! {
        <ThemeProvider>
            <PageRouter>
                <ThemedApp />
            </PageRouter>
        </ThemeProvider>
    }
}

#[function_component]
fn ThemedApp() -> Html {
    let navigator = use_navigator().unwrap();

    // Helper function for generating tabs
    fn page_option(current: &Page, page: Page, navigator: Navigator, state: UseStateHandle<Page>) -> ActionbarOption {
        let is_current = *current == page;
        ActionbarOption::new(
            page.name(),
            match page {
                Page::Home => Icon::Home,
                Page::Projects => Icon::Code,
                Page::Socials => Icon::SocialMedia,
                Page::Gallery => Icon::Camera,
                Page::Contact => Icon::Envelope,
            },
            Callback::from(move |_| {
                navigator.push(&page);
                state.set(page);
            }),
            is_current
        )
    }

    let ctx = use_context::<ThemeContext>().expect("No theme context supplied");
    let page_state = use_state_eq(||
        window()
            .location()
            .pathname()
            .ok()
            .and_then(|p| Page::recognize(&p))
            .or(Some(Page::Home))
            .unwrap()
    );
    let current_page = *page_state;

    html! {
        <>
            <div class="main">
                <Pages />
            </div>
            <Actionbar>
                {vec![
                    page_option(&current_page, Page::Home, navigator.clone(), page_state.clone()),
                    page_option(&current_page, Page::Projects, navigator.clone(), page_state.clone()),
                    page_option(&current_page, Page::Socials, navigator.clone(), page_state.clone()),
                    page_option(&current_page, Page::Gallery, navigator.clone(), page_state.clone()),
                    page_option(&current_page, Page::Contact, navigator, page_state),
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
        </>
    }
}