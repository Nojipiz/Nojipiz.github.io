use crate::languages::languages::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use yewdux::prelude::PersistentStore;
use yewdux_functional::use_store;

use crate::app::AppProperties;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section id="home" class="homeSection">
            <PresentationTitle />
            <canvas id={"uniqueElement"}> </canvas>
            <DonutAnimation />
        </section>
    }
}

#[function_component(DonutAnimation)]
fn donut_animation() -> Html {
    html! {
        <Donut/>
    }
}

#[function_component(PresentationTitle)]
fn presentation_title() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let presentation_text = get_home_content_text(store);
    html! {
        <p>
            {presentation_text[0]}<b>{presentation_text[1]} </b>
            <br/>
            {presentation_text[2]}
        </p>
    }
}

#[function_component(Donut)]
fn donnut_code() -> Html {
    let win = web_sys::window().unwrap();
    let document = win.document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("uniqueElement")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    // create a counter animation in the canvas that updates every second
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let mut counter = 0;
    let mut interval = win.set_interval_with_callback_and_timeout_and_arguments_0(
        move || {
            counter += 1;
            ctx.clear_rect(0.0, 0.0, canvas.width(), canvas.height());
            ctx.begin_path();
            ctx.arc(
                canvas.width() / 2.0,
                canvas.height() / 2.0,
                canvas.width() / 2.0,
                0.0,
                2.0 * std::f32::consts::PI,
            );
            ctx.stroke();
            ctx.fill_style = "white".into();
            ctx.fill_text(
                &format!("{}", counter),
                canvas.width() / 2.0,
                canvas.height() / 2.0,
            );
        },
        1000,
    );
    html! {}
}
