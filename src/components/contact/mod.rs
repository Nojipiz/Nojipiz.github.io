use crate::{app::AppProperties, languages::languages::get_contact_context_text};
use wasm_bindgen::JsCast;
use web_sys::{window, FormData, HtmlFormElement};
use yew::{html::onsubmit::Event, prelude::*};
use yewdux::prelude::use_store;

use self::form_information::FormInformation;

mod form_information;

#[function_component(Contact)]
pub fn contact() -> Html {
    let store = use_store::<AppProperties>();
    let text_content = get_contact_context_text(store);
    html! {
        <section id="contact" class="contact_section">
            <div class={"title_container"}>
                <div class={"line"}/>
                <h1>{text_content[0].to_uppercase()}</h1>
                <div class={"line"}/>
            </div>
            <ContactWaysContainer content={text_content}/>
            <ContactForm content={text_content}/>
        </section>
    }
}

#[function_component(ContactForm)]
fn contact_form(props: &ContactProps) -> Html {
    html! {
        <form class={"contact_form"} onsubmit={|e:Event| {
            e.prevent_default();
            let form:&HtmlFormElement = &e.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_information = FormInformation::from(FormData::new_with_form(form).unwrap());
            window().unwrap().open_with_url(&form_information.create_url()).unwrap();
        }}>
        <h1>{props.content[3].to_uppercase()}</h1>
            <div class={"inter_wrapper"}>
                <input type="text" id="formName" name="name" placeholder={props.content[4]}/>
                <input type="email" id="formEmail" name="email" placeholder={props.content[5]}/>
            </div>
            <input type="text" id="formSubject" name="subject" placeholder={props.content[6]}/>
            <input type="text" id="formMessage" name="message" placeholder={props.content[7]}/>
            <button type="submit">{props.content[8]}</button>
        </form>
    }
}

#[function_component(ContactWaysContainer)]
fn contact_ways_container(props: &ContactProps) -> Html {
    html! {
        <div class={"contact_ways_container"}>
        {get_contact_ways_list(props).iter().map(|way|{
        html!{
            <ContactWay
                image = {way.image.clone()}
                text = {way.text.clone()}
                contact_information = {way.contact_information.clone()}
                url = {way.url.clone()}
            />
            }
        }).collect::<Html>()}
        </div>
    }
}

fn get_contact_ways_list(props: &ContactProps) -> [ContactWayProps; 2] {
    [
        ContactWayProps {
            image: String::from("https://cdn-icons-png.flaticon.com/512/4542/4542248.png"),
            text: props.content[1].to_owned(),
            contact_information: String::from("nojipiz@gmail.com"),
            url: String::from("mailto:nojipiz@gmail.com?subject=Let's%20talk%20about%20a%20project!&body=Hi%20Orlando%2C%20i%20have%20the%20best%20idea%20for%20a%20project!"),
        },
        ContactWayProps {
            image: String::from("https://cdn-icons-png.flaticon.com/512/4542/4542152.png"),
            text: props.content[2].to_owned(),
            contact_information: String::from("3202010190"),
            url: String::from("https://wa.link/rno3oe"),
        },
    ]
}

#[function_component(ContactWay)]
fn contact_way(props: &ContactWayProps) -> Html {
    html!(
        <a class="contact_way" href={props.url.clone()} target={"_blank"}>
            <div class={"image_wrapper"}>
            <img src={props.image.clone()} alt={props.text.clone()} class={"contact_icon"}/>
            </div>
            <p class={"contact_information"}> {&props.contact_information} </p>
            <p> {&props.text.to_lowercase()} </p>
        </a>
    )
}

#[derive(Properties, PartialEq)]
struct ContactWayProps {
    image: String,
    text: String,
    contact_information: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct ContactProps {
    content: [&'static str; 9],
}
