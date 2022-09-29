use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::header::Header;
use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
pub struct AppProperties {
    pub language: String,
    pub color_scheme: String,
}

impl Default for AppProperties {
    fn default() -> Self {
        AppProperties {
            language: String::from("es"),
            color_scheme: String::from("dark"),
        }
    }
}
