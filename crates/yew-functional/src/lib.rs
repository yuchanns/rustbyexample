mod components;
mod counter;
mod layouts;

use layouts::home::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}
