use yew::prelude::*;

struct Section {
    name: String,
    section_id: String,
}

#[function_component(Header)]
pub fn nav() -> Html {
    let sections: [Section; 4] = [
        Section {
            name: String::from("Home"),
            section_id: String::from("/#home"),
        },
        Section {
            name: String::from("Porfolio"),
            section_id: String::from("/#portfolio"),
        },
        Section {
            name: String::from("About"),
            section_id: String::from("/#about"),
        },
        Section {
            name: String::from("Contact"),
            section_id: String::from("/#contact"),
        },
    ];

    html! {
        <nav>
        {
            sections.iter().map(|section| {
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
