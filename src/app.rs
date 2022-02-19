use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            //<Header />
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
