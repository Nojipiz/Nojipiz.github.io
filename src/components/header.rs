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

    let actual_section = use_state(|| 0);

    let change_section = {
        let actual_section = actual_section.clone();
        Callback::from(move |_| actual_section.set(*actual_section + 1))
    };

    html! {
        <>
        {*actual_section}
        <nav>
        {
            sections.iter().map(|section| {
            html!{<ul onclick={&change_section}> {&section.name} </ul>}
            }).collect::<Html>()
        }
        </nav>
        </>
    }
}
