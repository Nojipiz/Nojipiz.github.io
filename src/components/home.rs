use crate::languages::languages::*;
use web_sys::Window;
use yew::prelude::*;
use yewdux::prelude::PersistentStore;
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
        //Todo: Generate donut code in rust
        <Donut/>
    }
}

#[function_component(PresentationTitle)]
fn presentation_title() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let presentation_text = get_home_content_text(store);
    html! {
        <p>
            {presentation_text[0]}<b>{presentation_text[1]} </b>
            <br/>
            {presentation_text[2]}
        </p>
    }
}

#[function_component(Donut)]
fn donnut_code() -> Html {
    let win: Window = web_sys::window().unwrap();
    let res = win.set_interval_with_str_and_timeout_and_unused_0("console.log('Hello')", 1000);
    res.unwrap();
    html! {}
}
