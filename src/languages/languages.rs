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

pub fn get_about_context_text(store: StoreRef<BasicStore<AppProperties>>) -> [&'static str; 5] {
    let language: String = store
        .state()
        .map(|state| state.language.clone())
        .unwrap_or_default();
    match language.as_str() {
        "es" => [
            "Quien soy",
            "Soy estudiante de ingenieria de sistemas y desarrollador autodidacta,
            me encanta el café, la programación competitiva, el anime y personalizar 
            sistemas linux, me apasiona escribir codigo en todas sus formas",
            "Mis habilidades",
            "Trabajo",
            "Trabajo para darte el mejor codigo posible, las interfaces 
            minimalistas, esteticas, limpias y faciles de usar son parte 
            de mi trabajo.
            En un proyecto, mi objetivo principal es resolver tu problema, 
            el valor de mi trabajo es por proyecto, de esta forma me puedo 
            concentrar en el producto sin tener que contar las horas que paso en el :)",
        ],
        _ => [
            "Who i am",
            "I’m a systems engineering student and self-taught developer,
            I love coffee, competitive programming, anime and customizing 
            linux systems, passionate about writing code in every way.",
            "My skills",
            "My work",
            "I work to give you the most flawless code possible, the beautiful,
            clean, minimalist and easy-to-use interfaces are part of my work. In a project,
            my main objective is to solve your problem, the value of my work is per project, 
            so I can focus on finish it without count the hours :)",
        ],
    }
}
