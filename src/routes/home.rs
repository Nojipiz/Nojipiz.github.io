use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <section id="home">
        <p>{"home"} </p>
        </section>
        <section id="contact">
        <p>{"contact"} </p>
        </section>

        </>
    }
}
