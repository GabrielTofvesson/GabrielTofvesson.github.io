use gloo::utils::window;
use yew::prelude::*;
use yew_router::Routable;
use yew_router::prelude::Navigator;
use yew_router::prelude::use_navigator;
use crate::component::fa_icon::FontawesomeIcon;
use crate::page::Page;
use crate::page::PageRouter;
use crate::page::Pages;
use crate::theme::ThemeContext;
use crate::theme::ThemeMsg;
use crate::theme::ThemeProvider;
use crate::component::actionbar::{Actionbar, ActionbarOption};
use crate::theme::ThemeState;


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
    fn page_option(current: &Page, page: Page, navigator: Navigator, state: UseStateSetter<Page>) -> ActionbarOption {
        let is_current = *current == page;
        ActionbarOption::new(
            page.name(),
            match page {
                Page::Home => FontawesomeIcon::House,
                Page::Projects => FontawesomeIcon::Code,
                Page::Socials => FontawesomeIcon::ShareNodes,
                Page::Gallery => FontawesomeIcon::Camera,
                Page::Contact => FontawesomeIcon::Envelope,
            },
            Callback::from(move |_| {
                navigator.push(&page);
                state.set(page);
            }),
            is_current
        )
    }

    let ctx = use_context::<ThemeContext>().expect("No theme context supplied");
    let theme_icon = if ctx.theme == ThemeState::Dark { FontawesomeIcon::Sun } else { FontawesomeIcon::Moon };
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
            <Actionbar mobile_title="Menu" items={vec![
                page_option(&current_page, Page::Home, navigator.clone(), page_state.setter()),
                page_option(&current_page, Page::Projects, navigator.clone(), page_state.setter()),
                page_option(&current_page, Page::Socials, navigator.clone(), page_state.setter()),
                page_option(&current_page, Page::Gallery, navigator.clone(), page_state.setter()),
                page_option(&current_page, Page::Contact, navigator, page_state.setter()),
                ActionbarOption::new_opt(
                    None,
                    Some(theme_icon),
                    Some(Callback::from(move |_| ctx.dispatch(ThemeMsg::Toggle))),
                    true,
                    false,
                    false
                )
            ]}/>
        </>
    }
}