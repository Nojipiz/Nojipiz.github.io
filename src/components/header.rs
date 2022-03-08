use yew::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::*;

use crate::app::AppProperties;

#[function_component(Header)]
pub fn nav() -> Html {
    let store = use_store::<BasicStore<AppProperties>>();
    let language = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    html! {
        <nav>
        {
            get_nav_sections(language).iter().map(|section| {
            html!{
                <a class="" href={section.section_id.clone()}>
                {section.name.clone()}
                </a>
                }
            }).collect::<Html>()
        }
        </nav>
    }
}

fn get_nav_sections(language: String) -> Vec<Section> {
    let section_names: [&str; 4] = match language.as_str() {
        "es" => ["Home", "Portafolio", "Sobre", "Contacto"],
        _ => ["Home", "Portfolio", "About", "Contact"],
    };
    let section_ids: [&str; 4] = ["/#home", "/#portfolio", "/#about", "/#contact"];
    section_names
        .iter()
        .enumerate()
        .map(|(index, name)| Section {
            name: String::from(*name),
            section_id: String::from(section_ids[index]),
        })
        .collect::<Vec<Section>>()
}

struct Section {
    name: String,
    section_id: String,
}
