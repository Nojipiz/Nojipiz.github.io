use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html!(
         <footer>
            <p> {"2022 Orlando Vargas"} </p>
            <p> {"Colombia"} </p>
            <p> {"nojipiz@gmail.com"} </p>
         </footer>
    )
}
