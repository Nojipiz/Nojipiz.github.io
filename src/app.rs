use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::routes::{switch, AppRoute};

#[derive(Default, Clone)]
pub struct AppProperties {
    pub language: String,
    pub color_scheme: String,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}
