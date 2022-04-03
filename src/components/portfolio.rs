use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_portfolio_content_text};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_portfolio_content_text(store);
    html! {
        <section id="portfolio" class="portfolioSection">
        <h2 class="rigthTitle"> {content_text[0]} </h2>
        <PortfolioElement/>
        </section>
    }
}

#[function_component(PortfolioElement)]
fn get_portfolio_element() -> Html {
    let state = { use_async(async move { fetch_repo().await }) };
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };
    html! {
    <>
        <button onclick={onclick}> {"Hi mom"}</button>
        <p>{"element"}</p>
        <p>{match &state.data{
            Some(data) => &data.elements,
            None => "xd",
        }}</p>
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
    data: String,
    elements: String,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
}
