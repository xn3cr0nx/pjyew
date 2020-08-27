#![recursion_limit="10000"]

#[macro_use()]
extern crate log;
extern crate wasm_logger;
// extern crate web_logger;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
mod router;
mod routes;
mod utils;
mod media;

#[wasm_bindgen(start)]
pub fn run_app() {
    utils::set_panic_hook();
    // web_logger::init();
    wasm_logger::init(wasm_logger::Config::default());
    App::<app::App>::new().mount_to_body();
}
