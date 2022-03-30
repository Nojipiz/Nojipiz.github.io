use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yewdux::prelude::{BasicStore, PersistentStore};
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
    let repo = use_state(|| "jetli/yew-hooks".to_string());
    // Demo #1, manually call `run` to load data.
    let state = {
        let repo = repo.clone();
        use_async(async move { fetch_repo((*repo).clone()).await })
    };
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            // You can manually trigger to run in callback or use_effect.
            state.run();
        })
    };
    html! {
    <>
        <button onclick={onclick}> {"Hi mom"}</button>
        <p>{"element"}</p>
        <p>{match &state.data{
            Some(data) => &data.name,
            None => "xd",
        }}</p>
    </>
    }
}

async fn get_information() {
    match reqwest::get("https://dog.ceo/api/breeds/image/random").await {
        Ok(response) => {
            let body = response.text().await.unwrap();
        }
        Err(e) => {
            console::log_1(&JsValue::from_str(&format!("{:?}", e)));
        }
    }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    fetch::<Repo>(format!("https://api.github.com/repos/{}", repo)).await
}

/// You can use reqwest or other crates to fetch your api.
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
struct User {
    id: i32,
    login: String,
    avatar_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
    id: i32,
    name: String,
    full_name: String,
    description: String,
    owner: User,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
