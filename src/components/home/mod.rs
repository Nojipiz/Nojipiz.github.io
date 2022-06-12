use crate::languages::languages::*;
use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::app::AppProperties;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section id="home" class="home_section">
            <PresentationTitle />
            <MoonAnimation />
        </section>
    }
}

#[function_component(MoonAnimation)]
fn moon_animation() -> Html {
    html! {
        <>
            <canvas id="moonCanvas"> </canvas>
        </>
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
            <div class="home_lines_container">
                <div class="home_line"/>
                <div class="home_line"/>
            </div>
        </p>
    }
}
