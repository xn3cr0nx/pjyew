#[macro_use()]
extern crate web_logger;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

// static color: &str = "#252627";
// scrollbar-color: #202324 #454a4d
// background = "#292a2d";
// secondary background = "#3b3d42";
// border = "#4a4b50";
// dark color = #a9a9b3
// dark color secondary = #73747b
// text color = #e8e6e3 !important;

mod app;
mod components;
mod router;
mod routes;
mod utils;

// #[wasm_bindgen]
// pub fn run_app() -> Result<(), JsValue> {
//     utils::set_panic_hook();
//     web_logger::init();
//     yew::start_app::<app::App>();
//     Ok(())
// }

#[wasm_bindgen(start)]
pub fn run_app() {
    utils::set_panic_hook();
    web_logger::init();
    App::<app::App>::new().mount_to_body();
}
