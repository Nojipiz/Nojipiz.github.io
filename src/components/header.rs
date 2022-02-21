use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

struct Section {
    name: String,
    section_id: String,
}

#[function_component(Header)]
pub fn nav() -> Html {
    let sections: [Section; 4] = [
        Section {
            name: String::from("Home"),
            section_id: String::from("home"),
        },
        Section {
            name: String::from("Porfolio"),
            section_id: String::from("portfolio"),
        },
        Section {
            name: String::from("About"),
            section_id: String::from("about"),
        },
        Section {
            name: String::from("Contact"),
            section_id: String::from("contact"),
        },
    ];

    html! {
        <>
        <nav>
        {
            sections.iter().map(|section| {
            html!{
                <Link<AppRoute>
                    to={AppRoute::HomeSections { section: section.section_id.clone() }}>
                    { section.name.clone() }
                </Link<AppRoute>>
                }
            }).collect::<Html>()
        }
        </nav>
        </>
    }
}
