use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use crate::app::AppProperties;

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
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    let presentation_text: String = match language.as_str() {
        "es" => {
            "Soy estudiante de ingenieria de sistemas y desarrollador autodidacta,
            me encanta el café, la programación competitiva, el anime y personalizar 
            sistemas linux, me apasiona escribir codigo en todas sus formas"
        }
        _ => {
            "I’m a systems engineering student and self-taught developer,
            I love coffee, competitive programming, anime and customizing 
            linux systems, passionate about writing code in every way."
        }
    }
    .to_owned();
    html! {
        <>
            <div><p>{presentation_text} </p></div>
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
