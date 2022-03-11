use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use crate::app::AppProperties;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section id="home" class="homeSection">
        <PresentationTitle />
        <DonutAnimation />
        </section>
    }
}

#[function_component(DonutAnimation)]
fn donut_animation() -> Html {
    html! {
        <p>{"Here will be a Donut code"}</p>
        //Todo: Generate donut code in rust
    }
}

#[function_component(PresentationTitle)]
fn presentation_title() -> Html {
    let store = use_store::<BasicStore<AppProperties>>();
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    let presentation_text: [&str; 3] = match language.as_str() {
        "es" => [
            "Hola, mi nombre es ",
            "David",
            "Soy desarrollador de software",
        ],
        _ => ["Hi, I'm ", "David", "I'm a software developer"],
    };
    html! {
        <p>
            {presentation_text[0]}<b>{presentation_text[1]} </b>
            <br/>
            {presentation_text[2]}
        </p>
    }
}
