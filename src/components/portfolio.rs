use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{
    app::AppProperties,
    languages::languages::{get_portfolio_content_text, get_portfolio_project_content_text},
};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_portfolio_content_text(store);
    html! {
        <section id="portfolio" class="portfolioSection">
        <h2 class="rigthTitle"> {content_text[0]} </h2>
        <PortfolioElements/>
        </section>
    }
}

#[function_component(PortfolioElements)]
fn portfolio_elements() -> Html {
    let state = {
        use_async_with_options(
            async move { fetch_repo().await },
            UseAsyncOptions::enable_auto(),
        )
    };
    html! {
    <>
        {match &state.data{
            Some(data) => data.data.iter().map(|element| {
                html!{
                    <ProjectComponent project={element.clone()}/>
                }
            }).collect::<Html>(),
            None => html!{<p>{"F"}</p>},
        }}
    </>
    }
}

#[function_component(ProjectComponent)]
fn project_component(props: &ProjectProps) -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    //let project_content_text = get_portfolio_project_content_text(props.project, store);
    html! {
        <>
            <h1>{props.project.title.clone()}</h1>
            <p>{props.project.description.clone()}</p>
        </>
    }
}

async fn fetch_repo() -> Result<Projects, Error> {
    fetch::<Projects>(String::from(
        "https://raw.githubusercontent.com/Nojipiz/Nojipiz/main/Projects.json",
    ))
    .await
}

async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Projects {
    data: Vec<Project>,
}

#[derive(Clone, Properties, PartialEq)]
struct ProjectProps {
    project: Project,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
}
