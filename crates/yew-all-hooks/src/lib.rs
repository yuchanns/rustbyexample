mod app;
mod hooks;

use crate::app::AppStartup;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppStartup>::new().mount_to_body();
}
