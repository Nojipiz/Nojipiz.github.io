use crate::languages::languages::*;
use serde::de::IntoDeserializer;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement};
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

    let f = Closure::wrap(Box::new(move || {
        counter += 1;
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        ctx.begin_path();
        ctx.arc(
            (canvas.width() / 2) as f64,
            (canvas.height() / 2) as f64,
            (canvas.width() / 2) as f64,
            0.0,
            2.0 * std::f32::consts::PI as f64,
        );
        ctx.stroke();
        ctx.fill_text(
            &format!("{}", counter),
            (canvas.width() / 2) as f64,
            (canvas.height() / 2) as f64,
        );
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("uniqueElement")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(f.as_ref().unchecked_ref()));

    web_sys::window()
        .unwrap()
        .set_interval_with_callback(Some(f.as_ref().unchecked_ref()).unwrap());

    f.forget();
    html! {}
}
