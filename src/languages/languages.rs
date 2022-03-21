use yewdux::prelude::BasicStore;
use yewdux_functional::StoreRef;

use crate::app::AppProperties;

pub fn get_header_section_names(store: StoreRef<BasicStore<AppProperties>>) -> [&'static str; 4] {
    let language = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => ["home", "portafolio", "sobre", "contacto"],
        _ => ["home", "portfolio", "about", "contact"],
    }
}

pub fn get_home_content_text(store: StoreRef<BasicStore<AppProperties>>) -> [&'static str; 3] {
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => [
            "Hola, mi nombre es ",
            "David",
            "Soy desarrollador de software",
        ],
        _ => ["Hi, I'm ", "David", "I'm a software developer"],
    }
}

pub fn get_portfolio_content_text(store: StoreRef<BasicStore<AppProperties>>) -> [&'static str; 1] {
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => ["Portafolio"],
        _ => ["Portfolio"],
    }
}

pub fn get_about_context_text(store: StoreRef<BasicStore<AppProperties>>) -> [&'static str; 1] {
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => [
            "Soy estudiante de ingenieria de sistemas y desarrollador autodidacta,
            me encanta el café, la programación competitiva, el anime y personalizar 
            sistemas linux, me apasiona escribir codigo en todas sus formas",
        ],
        _ => [
            "I’m a systems engineering student and self-taught developer,
            I love coffee, competitive programming, anime and customizing 
            linux systems, passionate about writing code in every way.",
        ],
    }
}
