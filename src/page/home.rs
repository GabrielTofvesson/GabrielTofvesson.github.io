use comrak::{markdown_to_html, ComrakOptions};
use gloo::utils::document;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, Html, UseStateHandle, use_state, use_effect_with_deps, Properties, Children, use_context, Callback};
use yewprint::{Divider, Elevation, Card, Tag, Intent, Icon, Overlay};

use crate::{util::log, theme::{ThemeContext, ThemeState}, component::image_viewer::{ImageDescription, ImageViewer}};

#[derive(Debug)]
enum TagType {
    NaturalLanguage, CodeLanguage, Interest
}

#[derive(PartialEq)]
enum ImageSource {
    Link(String),
    Icon(Icon, Intent)
}

#[derive(PartialEq)]
struct ImageResource {
    source: ImageSource,
    clickable: bool
}

impl ImageResource {
    pub fn new_link(link: String, clickable: bool) -> Self {
        Self {
            source: ImageSource::Link(link),
            clickable
        }
    }

    pub fn new_icon(icon: Icon, intent: Intent, clickable: bool) -> Self {
        Self {
            source: ImageSource::Icon(icon, intent),
            clickable
        }
    }
}

#[derive(PartialEq)]
struct DescribedImage {
    pub image: ImageResource,
    pub description: &'static str
}

#[derive(Properties, PartialEq)]
struct HomeCardProps {
    #[prop_or_default]
    pub image: Option<DescribedImage>,
    pub children: Children
}

#[derive(Clone, PartialEq, Deserialize)]
struct GithubEntry {
    link: String,
    title: String,
    description: String
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <HomeTitle />
            <Divider />
            <div class="home-content">
                <Profile />
            </div>
        </>
    }
}

#[function_component]
fn HomeTitle() -> Html {
    html! {
        <div class="home-title">
            <h2>{"Gabriel Tofvesson"}</h2>
        </div>
    }
}

fn get_text_resource(file: String, on_result: impl (FnOnce(String) -> ()) + 'static) {
    wasm_bindgen_futures::spawn_local(async move {
        log(&format!("Fetching {file}"));
        let response = Request::get(file.as_str()).send().await;
        if let Ok(response) = response {
            if let Ok(text) = response.text().await {
                on_result(text);
            }
        }
    });
}

fn get_json_resource<T>(file: &'static str, on_result: impl FnOnce(Vec<T>) -> () + 'static) where T: for<'a> Deserialize<'a> {
    wasm_bindgen_futures::spawn_local(async move {
        log(&format!("Fetching {file}"));
        let response = Request::get(file).send().await;

        if let Ok(response) = response {
            if let Ok(value) = response.json::<Vec<T>>().await {
                on_result(value);
            }
        }
    });
}

#[function_component]
fn ProfileTags() -> Html {
    let natural_languages: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let code_languages: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let interests: UseStateHandle<Vec<String>> = use_state(|| vec![]);


    // TODO: Cache results
    {
        let natural_languages = natural_languages.clone();
        use_effect_with_deps(
            move |_| get_json_resource(
                "/res/languages.json",
                move |it| natural_languages.set(it)
            ),
            ()
        );
    }

    {
        let code_languages = code_languages.clone();
        use_effect_with_deps(
            move |_| get_json_resource(
                "/res/code.json",
                move |it| code_languages.set(it)
            ),
            ()
        );
    }

    {
        let interests = interests.clone();
        use_effect_with_deps(
            move |_| get_json_resource(
                "/res/interests.json",
                move |it| interests.set(it)
            ),
            ()
        );
    }

    let tags = vec![
        natural_languages.iter().map(|it| (it, TagType::NaturalLanguage)).collect::<Vec<(&String, TagType)>>(),
        code_languages.iter().map(|it| (it, TagType::CodeLanguage)).collect::<Vec<(&String, TagType)>>(),
        interests.iter().map(|it| (it, TagType::Interest)).collect::<Vec<(&String, TagType)>>()
    ].into_iter().flatten().map(|(tag, tag_type)| {
        html! {
            <Tag
                interactive=true
                minimal=true
                round=true
                intent={
                    match tag_type {
                        TagType::NaturalLanguage => Intent::Primary,
                        TagType::CodeLanguage => Intent::Warning,
                        TagType::Interest => Intent::Success
                    }
                }>
                {tag}
            </Tag>
        }
    }).collect::<Html>();

    html! {
        <div class="profiletags">{tags}</div>
    }
}

#[function_component]
fn Profile() -> Html {
    let profile_text = use_state(|| "".to_owned());
    {
        let profile_text = profile_text.clone();
        use_effect_with_deps(move |_| {
            get_text_resource("/res/profile.md".to_owned(), move |text| profile_text.set(markdown_to_html(&text, &ComrakOptions::default())));
        }, ());
    }

    html! {
        <HomeCard image={DescribedImage{ image: ImageResource::new_link("profile.jpg".to_owned(), true), description: "About me" }}>
            <ProfileTags />
            {Html::from_html_unchecked(profile_text.to_string().into())}
        </HomeCard>
    }
}

#[function_component]
fn HomeCard(props: &HomeCardProps) -> Html {
    let overlay_state = use_state(|| false);
    let open_overlay_state = overlay_state.clone();
    html! {
        <Card elevation={Elevation::Level3} interactive=true class="home-card" onclick={Callback::from(move |_| open_overlay_state.set(true))}>
            {
                if let Some(image) = &props.image {
                    html! {
                        <div class="home-tag">
                            {
                                match &image.image.source {
                                    ImageSource::Link(link) => html! {
                                        <img class="home-image circle" src={if link.starts_with("https?://") { link.to_string() } else { format!("/img/{}", link) }} />
                                    },
                                    ImageSource::Icon(icon, intent) => html! { <Icon {icon} {intent} size={20}/> }
                                }
                            }

                            {
                                if image.image.clickable {
                                    if let ImageSource::Link(link) = &image.image.source {
                                        html! {
                                            <ImageViewer
                                                images={vec![
                                                    ImageDescription::new(link, "This is me"),
                                                    ImageDescription::new(link, "This is also me"),
                                                    ImageDescription::new(link, "This is not me")
                                                ]}
                                                open={*overlay_state}
                                                onclose={Callback::from(move |_| overlay_state.set(false))} />
                                        }
                                    } else { html! {} }
                                } else {
                                    html! {}
                                }
                            }
                            
                            <b>{image.description}</b>
                        </div>
                    }
                } else { html! {} }
            }
            <div class="home-info">
                {props.children.clone()}
            </div>
        </Card>
    }
}

#[function_component]
fn Github() -> Html {
    let github_entries: UseStateHandle<Vec<GithubEntry>> = use_state(|| vec![]);
    {
        let github_entries = github_entries.clone();
        use_effect_with_deps(move |_| get_json_resource("/res/github.json", move |it| github_entries.set(it)), ());
    }

    let theme_state = use_context::<ThemeContext>().expect("Theme context");
    html! {
        <HomeCard image={ DescribedImage {
            image: ImageResource::new_link(format!("github/github-mark{}.png", if theme_state.theme == ThemeState::Dark { "-white" } else { "" }), false),
            description: "GitHub"
         } }>
            {
                github_entries.iter().map(|it| {
                    let link = it.link.clone();
                    html! {
                        <Card elevation={Elevation::Level3} interactive=true onclick={Callback::from(move |_| {
                            if let Err(_) = document().location().unwrap().set_href(link.as_str()) {
                                log("Couldn't change href");
                            }
                        })}>
                            <h3>{it.title.clone()}</h3>
                            <p>{it.description.clone()}</p>
                        </Card>
                    }
                }).collect::<Html>()
            }
        </HomeCard>
    }
}