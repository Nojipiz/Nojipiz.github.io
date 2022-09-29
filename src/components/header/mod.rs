use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    app::AppProperties, hooks::use_current_section::use_current_section,
    languages::languages::get_header_section_names,
};

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
    let (properties, dispatch_properties) = use_store::<AppProperties>();
    let language = properties.language.clone();
    let change_language = dispatch_properties.reduce_mut_callback(|state| {
        state.language = match state.language.as_str() {
            "en" => "es",
            _ => "en",
        }
        .to_owned();
    });
    let color_scheme = properties.color_scheme.clone();
    {
        let current_scheme = color_scheme.clone();
        use_effect(move || {
            update_body_class(&&current_scheme);
            || ()
        });
    }
    let change_color_scheme = dispatch_properties.reduce_mut_callback(|state| {
        state.color_scheme = match state.color_scheme.as_str() {
            "light" => "dark",
            _ => "light",
        }
        .to_owned();
    });
    html! {
        <div class={"options_wrapper"}>
            <button class={"property_button"} onclick={change_language}>
                <LanguageIcon/>
                {language}
            </button>
            <button class={"property_button"} onclick={change_color_scheme}>
                <ColorSchemeIcon/>
                {color_scheme}
            </button>
        </div>
    }
}

#[function_component(NavigationSections)]
fn nav_sections() -> Html {
    let current_section = use_current_section();
    let store = use_store::<AppProperties>();
    let section_ids: [&str; 4] = ["home", "portfolio", "about", "contact"];
    let section_names: [&str; 4] = get_header_section_names(store);
    html! {
        <nav class={"navigation_section"}>
        {
            section_names.iter().enumerate().map(|(index, name)| {
                let ref_formated = format!("/#{}", section_ids[index]);
                if current_section == {section_ids[index]} {
                    return html!{
                        <div class="current_container">
                            <a href={ref_formated}>
                                {name.to_uppercase()}
                            </a>
                            <div class="current_line"/>
                        </div>
                    }
                }
                return html!{
                    <a href={ref_formated}>
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

#[function_component(ColorSchemeIcon)]
fn color_scheme_icon() -> Html {
    html! {
    <svg width="16" height="16" fill="currentColor" class="icon" viewBox="0 0 16 16">
      <path d="M8 5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zm4 3a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zM5.5 7a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm.5 6a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z"/>
      <path d="M16 8c0 3.15-1.866 2.585-3.567 2.07C11.42 9.763 10.465 9.473 10 10c-.603.683-.475 1.819-.351 2.92C9.826 14.495 9.996 16 8 16a8 8 0 1 1 8-8zm-8 7c.611 0 .654-.171.655-.176.078-.146.124-.464.07-1.119-.014-.168-.037-.37-.061-.591-.052-.464-.112-1.005-.118-1.462-.01-.707.083-1.61.704-2.314.369-.417.845-.578 1.272-.618.404-.038.812.026 1.16.104.343.077.702.186 1.025.284l.028.008c.346.105.658.199.953.266.653.148.904.083.991.024C14.717 9.38 15 9.161 15 8a7 7 0 1 0-7 7z"/>
    </svg>
    }
}

#[function_component(LanguageIcon)]
fn language_icon() -> Html {
    html! {
    <svg width="16" height="16" fill="currentColor" class="icon" viewBox="0 0 16 16">
      <path d="M4.545 6.714 4.11 8H3l1.862-5h1.284L8 8H6.833l-.435-1.286H4.545zm1.634-.736L5.5 3.956h-.049l-.679 2.022H6.18z"/>
      <path d="M0 2a2 2 0 0 1 2-2h7a2 2 0 0 1 2 2v3h3a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2v-3H2a2 2 0 0 1-2-2V2zm2-1a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h7a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H2zm7.138 9.995c.193.301.402.583.63.846-.748.575-1.673 1.001-2.768 1.292.178.217.451.635.555.867 1.125-.359 2.08-.844 2.886-1.494.777.665 1.739 1.165 2.93 1.472.133-.254.414-.673.629-.89-1.125-.253-2.057-.694-2.82-1.284.681-.747 1.222-1.651 1.621-2.757H14V8h-3v1.047h.765c-.318.844-.74 1.546-1.272 2.13a6.066 6.066 0 0 1-.415-.492 1.988 1.988 0 0 1-.94.31z"/>
    </svg>
    }
}
