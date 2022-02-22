use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section id="home" class="homeSection">
        <PresentationTitle />
        <DonutAnimation />
        </section>
    }
}

#[function_component(DonutAnimation)]
fn donut_animation() -> Html {
    html! {
        <p>{"Here will be a Donut code"}</p>
    }
}

#[function_component(PresentationTitle)]
fn presentation_title() -> Html {
    html! {
        <p>
        {"Hola, soy David, soy desarrollador de software"}
        </p>
    }
}
