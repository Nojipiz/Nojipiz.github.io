use yewdux::prelude::BasicStore;
use yewdux_functional::StoreRef;

use crate::app::AppProperties;

pub fn get_home_content_text(store: StoreRef<BasicStore<AppProperties>>) -> [String; 3] {
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
    .map(|element| element.to_owned())
}
