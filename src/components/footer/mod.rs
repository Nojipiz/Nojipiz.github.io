use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_footer_context_text};

#[function_component(Footer)]
pub fn footer() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_footer_context_text(store);
    html!(
         <footer class={"main_footer"}>
            <p> {content_text[0]} </p>
            <p> {content_text[1]} </p>
            <p> {content_text[2]} </p>
         </footer>
    )
}
