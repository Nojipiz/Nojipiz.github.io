use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlElement};
use yew_hooks::{use_raf_state, use_window_scroll};

pub fn use_current_section() -> String {
    let position = use_window_scroll();
    let current_section = use_raf_state(|| "");
    let document: Document = window().unwrap().document().unwrap();
    let all_sections = document.query_selector_all("section").unwrap();
    {
        for index in 0..all_sections.length() {
            let section: HtmlElement = all_sections
                .get(index)
                .unwrap()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .clone();
            if position.1 >= f64::from(section.offset_top().clone() - 150) {
                let string_box = Box::leak(section.id().into_boxed_str());
                current_section.set(string_box);
            }
        }
    }
    String::from(*current_section)
}
