use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_about_context_text};

#[function_component(About)]
pub fn about() -> Html {
    let store = use_store::<BasicStore<AppProperties>>();
    let content_text = get_about_context_text(store);
    html! {
        <section id="about" class="aboutSection">
        {"About"}
        <WhoIAm content={content_text}/>
        <MySkills content={content_text}/>
        <MyWork content={content_text}/>
        </section>
    }
}

#[function_component(WhoIAm)]
fn who_i_am_component(props: &AboutProps) -> Html {
    html! {
        <>
            <div>
                <h1>{props.content[0]}</h1>
                <p>{props.content[1]}</p>
            </div>
            <img src="https://avatarfiles.alphacoders.com/229/229959.png"/>
        </>
    }
}

#[function_component(MySkills)]
fn my_skills(props: &AboutProps) -> Html {
    html! {
        <>
        <h1>{props.content[2]}</h1>
        </>
    }
}

#[function_component(MyWork)]
fn my_work(props: &AboutProps) -> Html {
    html! {
        <>
            <h1>{props.content[3]}</h1>
            <p>{props.content[4]}</p>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct AboutProps {
    content: [&'static str; 5],
}
