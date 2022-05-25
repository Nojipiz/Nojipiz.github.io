use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_contact_context_text};

#[function_component(Contact)]
pub fn contact() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let text_content = get_contact_context_text(store);
    html! {
        <section id="contact" class="contact_section">
            <ContactWaysContainer content={text_content}/>
            <ContactForm content={text_content}/>
        </section>
    }
}

#[function_component(ContactForm)]
fn contact_form(props: &ContactProps) -> Html {
    html! {
        <div>
        <h1>{props.content[3]}</h1>
        <form>
            <input type="text" id="formName" name="name" placeholder={props.content[4]}/>
            <input type="email" id="formEmail" name="email" placeholder={props.content[5]}/>
            <input type="text" id="formSubject" name="subject" placeholder={props.content[6]}/>
            <input type="text" id="formMessage" name="message" placeholder={props.content[7]}/>
            <input type="submit" value={props.content[8]}/>
        </form>
        </div>
    }
}

#[function_component(ContactWaysContainer)]
fn contact_ways_container(props: &ContactProps) -> Html {
    html! {
    get_contact_ways_list(props).iter().map(|way|{
        html!{
            <ContactWay
                image = {way.image.clone()}
                text = {way.text.clone()}
                contact_information = {way.contact_information.clone()}
            />
            }
        }).collect::<Html>()
    }
}

fn get_contact_ways_list(props: &ContactProps) -> [ContactWayProps; 2] {
    [
        ContactWayProps {
            image: String::from("img url"),
            text: props.content[1].to_owned(),
            contact_information: String::from("nojipiz@gmail.com"),
        },
        ContactWayProps {
            image: String::from("img url"),
            text: props.content[2].to_owned(),
            contact_information: String::from("320201029"),
        },
    ]
}

#[function_component(ContactWay)]
fn contact_way(props: &ContactWayProps) -> Html {
    html!(
        <div class="contactWayContainer">
            <p> {&props.contact_information} </p>
            <p> {&props.text} </p>
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct ContactWayProps {
    image: String,
    text: String,
    contact_information: String,
}

#[derive(Properties, PartialEq)]
struct ContactProps {
    content: [&'static str; 9],
}
