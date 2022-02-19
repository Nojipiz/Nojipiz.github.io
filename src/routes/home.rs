use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <a class="crab-gif"> </a>
        <div class="app">
        <p class="tempInfoText">
        <b>{"Website "}  {"Under Construction"} </b>
        </p>
        <p> {"The programmer is drinking a cup of coffee and will finish this site soon."} </p>
        <a class="webassembly-logo"> </a>
            <p> {"by Orlando Vargas"}</p>
        </div>
        </>
    }
}
