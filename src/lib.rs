pub mod app;
pub mod components;
pub mod fetch;
pub mod hooks;
pub mod languages;
pub mod resources;
pub mod routes;

use wasm_bindgen::prelude::*;
use app::App;

#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
