use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::*;

use crate::app::AppProperties;

#[function_component(Header)]
pub fn nav() -> Html {
    html! {
        <header>
            <NavigationSections />
            <PropertiesButtons/>
       </header>
    }
}

#[function_component(PropertiesButtons)]
fn get_properties_buttons() -> Html {
    let store = use_store::<BasicStore<AppProperties>>();
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
    let change_color_scheme = store.dispatch().reduce_callback(|state| {
        state.color_scheme = match state.color_scheme.as_str() {
            "light" => "dark",
            _ => "light",
        }
        .to_owned()
    });
    html! {
        <div>
            <button onclick={change_language}>{language}</button>
            <button onclick={change_color_scheme}>{color_scheme}</button>
        </div>
    }
}

#[function_component(NavigationSections)]
fn nav_sections() -> Html {
    let section_ids: [&str; 4] = ["/#home", "/#portfolio", "/#about", "/#contact"];
    let section_names: [&str; 4] = get_section_names();
    html! {
        <nav>
        {
            section_names.iter().enumerate().map(|(index, name)| {
            html!{
                <a class="" href={section_ids[index]}>
                {name}
                </a>
                }
            }).collect::<Html>()
        }
        </nav>

    }
}

fn get_section_names() -> [&'static str; 4] {
    let store = use_store::<BasicStore<AppProperties>>();
    let language = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => ["Home", "Portafolio", "Sobre", "Contacto"],
        _ => ["Home", "Portfolio", "About", "Contact"],
    }
}
