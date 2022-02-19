use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Header)]
pub fn nav() -> Html {
    html! {
        <nav>
            <ul>
                <li><Link<AppRoute> to={AppRoute::Home} classes="headerRouteElement" >{ "Home" }</Link<AppRoute>></li>
            </ul>
        </nav>
    }
}
