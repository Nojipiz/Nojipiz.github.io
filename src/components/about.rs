use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_about_context_text};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="aboutSection">
        {"About"}
        <WhoIAm />
        </section>
    }
}

#[function_component(WhoIAm)]
fn who_i_am_component() -> Html {
    let store = use_store::<BasicStore<AppProperties>>();
    let content_text = get_about_context_text(store);
    html! {
        <>
            <div><p>{content_text[0]} </p></div>
            <img src="https://avatarfiles.alphacoders.com/229/229959.png"/>
        </>
    }
}

#[function_component(MySkills)]
fn my_skills() -> Html {
    html! {
        <>
        <h1>{"My Skills"}</h1>
        </>
    }
}
