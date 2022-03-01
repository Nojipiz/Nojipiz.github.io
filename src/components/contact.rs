use yew::prelude::*;

#[function_component(Contact)]
pub fn contact(props: &ContactProps) -> Html {
    let language = use_state(|| String::from(&props.lang));
    let click = {
        let language = language.clone();
        Callback::from(move |_| language.set(String::from("hola")))
    };
    html! {
        <section id="contact" class="contactSection">
        <ContactWaysContainer />
        <ContactForm />
        <p> {"Language"} </p>
        <p> {(*language).clone()} </p>
        <button onclick={click}>{"Buenas"}</button>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContactProps {
    pub lang: String,
}

#[function_component(ContactForm)]
fn contact_form() -> Html {
    html! {
        <div>
        <h1>{"Contact Form"}</h1>
        <form>
            <input type="text" id="formName" name="name" placeholder="Your name.."/>
            <input type="email" id="formEmail" name="email" placeholder="Your email.."/>
            <input type="text" id="formSubject" name="subject" placeholder="Your subject.."/>
            <input type="text" id="formMessage" name="message" placeholder="Your message.."/>
            <input type="submit" value={"Send"}/>
        </form>
        </div>
    }
}

#[function_component(ContactWaysContainer)]
fn contact_ways_container() -> Html {
    let contact_ways_list = get_contact_ways_list();
    html! {
    contact_ways_list.iter().map(|way|{
        html!{
            <ContactWay
                image = {way.image.clone()}
                title = {way.title.clone()}
                contact_information = {way.contact_information.clone()}
            />
            }
        }).collect::<Html>()
    }
}

#[function_component(ContactWay)]
fn contact_way(props: &ContactWayProps) -> Html {
    html!(
        <div class="contactWayContainer">
            <h1> {&props.title} </h1>
            <p> {&props.contact_information} </p>
        </div>
    )
}

fn get_contact_ways_list() -> [ContactWayProps; 2] {
    [
        ContactWayProps {
            image: String::from("img url"),
            title: String::from("Telefono"),
            contact_information: String::from("320201029"),
        },
        ContactWayProps {
            image: String::from("img url"),
            title: String::from("Email"),
            contact_information: String::from("davidor2001"),
        },
    ]
}

#[derive(Properties, PartialEq)]
struct ContactWayProps {
    image: String,
    title: String,
    contact_information: String,
}
