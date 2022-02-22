use yew::prelude::*;
use yew_router::prelude::*;

pub mod main;

use main::Main;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Main,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Main => html! { <Main /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
