use crate::components::about::About;
use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::home::Home;
use crate::components::portfolio::Portfolio;
use yew::prelude::*;

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Home />
            <Portfolio />
            <About />
            <Contact lang="es"/>
            <Footer />
            </>
        }
    }
}
