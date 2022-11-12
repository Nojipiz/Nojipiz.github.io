use crate::{
    app::AppProperties, languages::languages::get_about_context_text, resources::resources::*,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(About)]
pub fn about() -> Html {
    let store = use_store::<AppProperties>();
    let content_text = get_about_context_text(store);
    html! {
        <section id="about" class="about_section">
        <h2 class="left_title"> {content_text[6].to_uppercase()} </h2>
        <WhoIAm content={content_text}/>
        <MyWork content={content_text}/>
        <MySkills />
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
            <img class={"profile_photo"} src="https://github.com/Nojipiz/Nojipiz/blob/main/resources/PhotoTransparent.png?raw=true"/>
        </div>
    }
}

#[function_component(MySkills)]
fn my_skills() -> Html {
    html! {
        <div class={"my_skills_container"}>
            <div class={"skills_topics_container"}>
                <SkillTopic icon_url={get_web_icon()} title={"Frontend"} description={"WEB"}/>
                <SkillTopic icon_url={get_server_icon()} title={"Backend"} description={"WEB"}/>
                <SkillTopic icon_url={get_mobile_icon()} title={"Mobile"} description={"WEB"}/>
                <SkillTopic icon_url={get_serverless_icon()} title={"Serverless"} description={"WEB"}/>
            </div>
        </div>
    }
}

#[function_component(SkillTopic)]
fn skill_topic(props: &SkillTopicProps) -> Html {
    html! {
        <div class={"skill_topic_container"}>
            <img src={props.icon_url.to_string()} class={"skill_icon"}/>
            <h2 class={"skill_title"}>{props.title.clone()}</h2>
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
struct SkillTopicProps {
    title: String,
    icon_url: String,
    description: &'static str,
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
