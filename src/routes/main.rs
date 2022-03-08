use crate::components::about::About;
use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::home::Home;
use crate::components::portfolio::Portfolio;

use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <>
        <Home />
        <Portfolio />
        <About />
         <Contact lang={"es"}/>
        <Footer />
        </>
    }
}
