use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_about_context_text};

#[function_component(About)]
pub fn about() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let content_text = get_about_context_text(store);
    html! {
        <section id="about" class="about_section">
        <h2 class="left_title"> {content_text[6].to_uppercase()} </h2>
        <WhoIAm content={content_text}/>
        <MySkills content={content_text}/>
        <MyWork content={content_text}/>
        <ToolBox content={content_text}/>
        </section>
    }
}

#[function_component(WhoIAm)]
fn who_i_am_component(props: &AboutProps) -> Html {
    html! {
        <div class={"who_i_am_container"}>
            <div class={"information_container"}>
                <div class={"title_container"}>
                    <div class="line"/>
                    <h1>{props.content[0].to_uppercase()}</h1>
                    <div class="line_bottom"/>
                </div>
                <p>{props.content[1]}</p>
            </div>
            <img src="https://avatarfiles.alphacoders.com/229/229959.png"/>
        </div>
    }
}

#[function_component(MySkills)]
fn my_skills(props: &AboutProps) -> Html {
    html! {
        <div class={"my_skills_container"}>
            <h1>{props.content[2].to_uppercase()}</h1>
            <p>{"Web / Mobile / Languages (TODO)"}</p>
        </div>
    }
}

#[function_component(MyWork)]
fn my_work(props: &AboutProps) -> Html {
    html! {
        <div class={"my_work_container"}>
            <img src="https://avatarfiles.alphacoders.com/229/229959.png"/>
            <div class={"content_wrapper"}>
                <div class={"title_container"}>
                    <div class="line"/>
                    <h1>{props.content[3].to_uppercase()}</h1>
                    <div class="line_bottom"/>
                </div>
                <p>{props.content[4]}</p>
           </div>
        </div>
    }
}

#[function_component(ToolBox)]
fn tool_box(props: &AboutProps) -> Html {
    html! {
        <div class={"tool_box_container"}>
            <h1>{props.content[5].to_uppercase()}</h1>
            <p>{"Linux / Figma (TODO)"}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AboutProps {
    content: [&'static str; 7],
}
