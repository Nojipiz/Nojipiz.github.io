use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{
    app::AppProperties,
    fetch::fetch::{fetch_from_url, Error},
    languages::languages::{get_portfolio_content_text, get_project_fields},
};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_portfolio_content_text(store);
    html! {
        <section id="portfolio" class="portfolio_section">
            <h2 class="rigth_title"> {content_text[0].to_uppercase()} </h2>
            <PortfolioElements/>
        </section>
    }
}

#[function_component(PortfolioElements)]
fn portfolio_elements() -> Html {
    let state = {
        use_async_with_options(
            async move { fetch_projects_repo().await },
            UseAsyncOptions::enable_auto(),
        )
    };
    html! {
    <>{if state.loading{
        html!{
            <>{"Loading..."}</>
        }
    }
    else {
        match &state.data{
        Some(data) => data.data.iter().map(|element| {
            html!{
                <ProjectComponent project={element.clone()}/>
            }
        }).collect::<Html>(),
        None => html!{<p>{"No data sorry :("}</p>}
        }
    }
    }</>
    }
}

#[function_component(ProjectComponent)]
fn project_component(props: &ProjectProps) -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let project = get_project_fields(props.project.clone(), store);
    html! {
        <div class={"project_container"}>
            <img src={project[2].clone()}/>
            <h1>{project[0].clone()}</h1>
            <p>{project[1].clone()}</p>
        </div>
    }
}

async fn fetch_projects_repo() -> Result<Projects, Error> {
    fetch_from_url::<Projects>(String::from(
        "https://raw.githubusercontent.com/Nojipiz/Nojipiz/main/Projects.json",
    ))
    .await
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
    pub title_es: String,
    pub description: String,
    pub description_es: String,
    pub img: String,
}
