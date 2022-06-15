use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::*;

use crate::{app::AppProperties, languages::languages::get_header_section_names};

#[function_component(Header)]
pub fn nav() -> Html {
    html! {
        <header class={"main_header"}>
            <div class={"header_wrapper"}>
                <NavigationSections />
                <PropertiesButtons/>
            </div>
       </header>
    }
}

#[function_component(PropertiesButtons)]
fn get_properties_buttons() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let language = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    let change_language = store.dispatch().reduce_callback(|state| {
        state.language = match state.language.as_str() {
            "en" => "es",
            _ => "en",
        }
        .to_owned();
    });
    let color_scheme = store
        .state()
        .map(|state| state.color_scheme.clone())
        .unwrap_or_default();
    {
        let current_scheme = color_scheme.clone();
        use_effect(move || {
            update_body_class(&&current_scheme);
            || ()
        });
    }
    let change_color_scheme = store.dispatch().reduce_callback(|state| {
        state.color_scheme = match state.color_scheme.as_str() {
            "light" => "dark",
            _ => "light",
        }
        .to_owned();
    });
    html! {
        <div class={"options_wrapper"}>
            <button onclick={change_language}>{language}</button>
            <button onclick={change_color_scheme}>{color_scheme}</button>
        </div>
    }
}

#[function_component(NavigationSections)]
fn nav_sections() -> Html {
    let store = use_store::<PersistentStore<AppProperties>>();
    let section_ids: [&str; 4] = ["/#home", "/#portfolio", "/#about", "/#contact"];
    let section_names: [&str; 4] = get_header_section_names(store);
    html! {
        <nav class={"navigation_section"}>
        {
            section_names.iter().enumerate().map(|(index, name)| {
            html!{
                <a href={section_ids[index]}>
                {name.to_uppercase()}
                </a>
                }
            }).collect::<Html>()
        }
        </nav>
    }
}

fn update_body_class(color_scheme: &str) {
    let window = web_sys::window().expect("I can't find your window :(");
    let document = window
        .document()
        .expect("There isn't a document in your window :(");
    let body = document
        .body()
        .expect("Sorry, I can't find the page body :(");
    body.set_class_name(match color_scheme {
        "dark" => "theme_dark",
        _ => "",
    })
}
