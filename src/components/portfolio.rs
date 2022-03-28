use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yewdux::prelude::{BasicStore, PersistentStore};
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_portfolio_content_text};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_portfolio_content_text(store);
    //spawn_local(get_information());
    html! {
        <section id="portfolio" class="portfolioSection">
        <h2 class="rigthTitle"> {content_text[0]} </h2>
        <PortfolioElement/>
        </section>
    }
}

#[function_component(PortfolioElement)]
fn get_portfolio_element() -> Html {
    let portfolio_store = use_store::<BasicStore<PortfolioStore>>();
    let info = portfolio_store
        .state()
        .map(|state| state.message.clone())
        .unwrap_or_default();
    html! {
        <p>{info}</p>
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

#[derive(Clone, Default, Debug)]
struct PortfolioStore {
    message: String,
    status: String,
}
