use crate::{app::AppProperties, components::portfolio::Project};
use std::rc::Rc;
use yewdux::prelude::*;

pub fn get_header_section_names(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 4] {
    let language = store.0.language.clone();
    match language.as_str() {
        "es" => ["home", "portafolio", "sobre", "contacto"],
        _ => ["home", "portfolio", "about", "contact"],
    }
}

pub fn get_home_content_text(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 3] {
    let language: String = store.0.language.clone();
    match language.as_str() {
        "es" => [
            "Hola, mi nombre es ",
            "Orlando",
            "Soy desarrollador de software",
        ],
        _ => ["Hello, I'm ", "Orlando", "I'm a software developer"],
    }
}

pub fn get_portfolio_content_text(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 1] {
    let language: String = store.0.language.clone();
    match language.as_str() {
        "es" => ["Portafolio"],
        _ => ["Portfolio"],
    }
}

pub fn get_project_fields(
    project: Project,
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [String; 4] {
    let language: String = store.0.language.clone();
    match language.as_str() {
        "es" => [
            project.title_es,
            project.description_es,
            project.img,
            project.background_img,
        ],
        _ => [
            project.title,
            project.description,
            project.img,
            project.background_img,
        ],
    }
}

pub fn get_about_context_text(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 7] {
    let language: String = store.0.language.clone();
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
            "Herramientas",
            "Sobre",
        ],
        _ => [
            "Who i am",
            "I’m a systems engineering student and self-taught developer,
            I love coffee, competitive programming, anime and customizing 
            linux systems, passionate about writing code in every way.",
            "My skills",
            "Work",
            "I work to give you the most flawless code possible, the beautiful,
            clean, minimalist and easy-to-use interfaces are part of my work. In a project,
            my main objective is to solve your problem, the value of my work is per project, 
            so I can focus on finish it without count the hours :)",
            "Toolbox",
            "About",
        ],
    }
}

pub fn get_contact_context_text(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 9] {
    let language: String = store.0.language.clone();
    match language.as_str() {
        "es" => [
            "Contacto",
            "Escribeme",
            "Hablemos",
            "Formulario de contacto",
            "Tu nombre",
            "Tu email",
            "Asunto",
            "Tu mensaje",
            "Enviar",
        ],
        _ => [
            "Contact",
            "Mail me",
            "Lets talk",
            "Contact form",
            "Your name",
            "Your email",
            "Subject",
            "Your message",
            "Send",
        ],
    }
}

pub fn get_footer_context_text(
    store: (Rc<AppProperties>, Dispatch<AppProperties>),
) -> [&'static str; 5] {
    let language: String = store.0.language.clone();
    match language.as_str() {
        "es" => [
            "Orlando Vargas",
            "Colombia",
            "nojipiz@gmail.com",
            "Esta pagina fué escrita en",
            "y compilada a",
        ],
        _ => [
            "Orlando Vargas",
            "Colombia",
            "nojipiz@gmail.com",
            "This page han been written in",
            "and compiled to",
        ],
    }
}
