#![recursion_limit = "1024"]
extern crate common;
extern crate console_error_panic_hook;
mod global;
mod ui;
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // enables ore helpful stack traces
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<ui::app::app::App>();
    Ok(())
}
