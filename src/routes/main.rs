use std::rc::Rc;

use crate::components::about::About;
use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::home::Home;
use crate::components::portfolio::Portfolio;

use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[derive(Clone, Default)]
pub struct MyState {
    count: u32,
}

pub struct Main {
    state: Rc<MyState>,
    dispatch: Dispatch<BasicStore<MyState>>,
}

pub enum Msg {
    State(Rc<MyState>),
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.state.count;
        let onclick = self.dispatch.reduce_callback(|s| s.count += 1);
        html! {
            <>
            <button onclick={onclick}>{"Buenas"}</button>
            <Home />
            <Portfolio />
            <About />
            <Contact lang={count.to_string()}/>
            <Footer />
            </>
        }
    }
}
