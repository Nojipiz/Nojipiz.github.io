use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;

use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/#:section")]
    HomeSections { section: String },
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::HomeSections { section } => html! {<p> {section} </p>},
        AppRoute::Home => html! { <Home /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
