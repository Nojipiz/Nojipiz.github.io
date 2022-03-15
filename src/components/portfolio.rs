use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use crate::app::AppProperties;

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    html! {
        <section id="portfolio" class="portfolioSection">
        <h2 class="rigthTitle"> {get_portfolio_title()} </h2>
        <PortfolioElement/>
        <h2 class="leftTitle"> {get_portfolio_title()} </h2>
        </section>
    }
}

fn get_portfolio_title() -> String {
    let store = use_store::<BasicStore<AppProperties>>();
    let language = store
        .state()
        .map(|s| s.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => "Portafolio",
        _ => "Portfolio",
    }
    .to_owned()
}

#[function_component(PortfolioElement)]
fn get_portfolio_element() -> Html {
    html! {
        <p>{"Todo: Here will be a portfolio element loaded from github page"}</p>
    }
}
