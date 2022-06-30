use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{
    app::AppProperties,
    languages::languages::get_about_context_text,
    resources::resources::{
        get_android_icon, get_docker_icon, get_firebase_icon, get_java_icon, get_javascript_icon,
        get_kotlin_icon, get_linux_icon, get_mongo_icon, get_postgre_icon, get_react_icon,
        get_rust_icon, get_spring_icon, get_typescript_icon,
    },
};

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
        <div class={"title_container"}>
            <div class="line"/>
            <h1>{props.content[5].to_uppercase()}</h1>
            <div class="line"/>
        </div>
            <div class={"tool_box"}>
                {get_tools_list().iter().map(|tool| {
                    html! {
                        <div class={"tool_box_item"}>
                            <img src={tool.icon.clone()}/>
                            <span>{tool.name}</span>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AboutProps {
    content: [&'static str; 7],
}

fn get_tools_list() -> Vec<Tool> {
    vec![
        Tool {
            name: "Android",
            icon: get_android_icon(),
        },
        Tool {
            name: "Kotlin",
            icon: get_kotlin_icon(),
        },
        Tool {
            name: "Java",
            icon: get_java_icon(),
        },
        Tool {
            name: "Spring",
            icon: get_spring_icon(),
        },
        Tool {
            name: "Typescript",
            icon: get_typescript_icon(),
        },
        Tool {
            name: "Javascript",
            icon: get_javascript_icon(),
        },
        Tool {
            name: "React",
            icon: get_react_icon(),
        },
        Tool {
            name: "Firebase",
            icon: get_firebase_icon(),
        },
        Tool {
            name: "MongoDb",
            icon: get_mongo_icon(),
        },
        Tool {
            name: "PostgresSQL",
            icon: get_postgre_icon(),
        },
        Tool {
            name: "Rust",
            icon: get_rust_icon(),
        },
        Tool {
            name: "Linux",
            icon: get_linux_icon(),
        },
        Tool {
            name: "Docker",
            icon: get_docker_icon(),
        },
    ]
}

#[derive(Properties, PartialEq)]
struct Tool {
    name: &'static str,
    icon: String,
}
