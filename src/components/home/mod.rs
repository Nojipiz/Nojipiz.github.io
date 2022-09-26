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
        <div class={"moon_container"}>
            <canvas id="moonCanvas"/>
        </div>
    }
}

#[function_component(PresentationTitle)]
fn presentation_title() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let presentation_text = get_home_content_text(store);
    html! {
        <div class={"home_title_container"}>
            <h1 class="presentation_title">
            {presentation_text[0]}<b>{presentation_text[1]} </b>
            <br/>
            {presentation_text[2]}
            </h1>
            <div class="home_lines_container">
                <div class="home_line"/>
                <div class="home_line"/>
            </div>
        </div>
    }
}
