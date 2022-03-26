use yew::prelude::*;
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
        <h2 class="leftTitle"> {content_text[0]} </h2>
        </section>
    }
}

#[function_component(PortfolioElement)]
fn get_portfolio_element() -> Html {
    html! {
        <p>{"Todo: Here will be a portfolio element loaded from github page"}</p>
    }
}
