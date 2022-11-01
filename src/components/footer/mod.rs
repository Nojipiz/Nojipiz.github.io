use crate::{app::AppProperties, languages::languages::get_footer_context_text};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Footer)]
pub fn footer() -> Html {
    let repository_url = String::from("https://github.com/Nojipiz/Nojipiz.github.io");
    let store = use_store::<AppProperties>();
    let content_text = get_footer_context_text(store);

    html!(
         <footer class={"main_footer"}>
            <p> {content_text[0]} </p>
            <p> {content_text[1]} </p>
            <p> {content_text[2]} </p>
            <a href={repository_url} class={"wasm_information"}>
                <p> {content_text[3]} </p>
                <img src={"https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png"}
                    class={"tech_logo"}/>
                <p> {content_text[4]} </p>
                <img src={"https://upload.wikimedia.org/wikipedia/commons/1/1f/WebAssembly_Logo.svg"}
                    class={"tech_logo"}/>
            </a>
         </footer>
    )
}
