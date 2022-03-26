use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::{app::AppProperties, languages::languages::get_contact_context_text};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="contactSection">
            <ContactWaysContainer />
            <ContactForm />
        </section>
    }
}

#[function_component(ContactForm)]
fn contact_form() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let contact_content_text = get_contact_context_text(store);
    html! {
        <div>
        <h1>{contact_content_text[3]}</h1>
        <form>
            <input type="text" id="formName" name="name" placeholder={contact_content_text[4]}/>
            <input type="email" id="formEmail" name="email" placeholder={contact_content_text[5]}/>
            <input type="text" id="formSubject" name="subject" placeholder={contact_content_text[6]}/>
            <input type="text" id="formMessage" name="message" placeholder={contact_content_text[7]}/>
            <input type="submit" value={contact_content_text[8]}/>
        </form>
        </div>
    }
}

#[function_component(ContactWaysContainer)]
fn contact_ways_container() -> Html {
    html! {
    get_contact_ways_list().iter().map(|way|{
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

fn get_contact_ways_list() -> [ContactWayProps; 2] {
    let store = use_store::<PersistentStore<AppProperties>>();
    let contact_content_text = get_contact_context_text(store);
    [
        ContactWayProps {
            image: String::from("img url"),
            text: contact_content_text[1].to_owned(),
            contact_information: String::from("nojipiz@gmail.com"),
        },
        ContactWayProps {
            image: String::from("img url"),
            text: contact_content_text[2].to_owned(),
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
