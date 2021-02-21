#![recursion_limit = "256"]

mod components;
mod utils;

use crate::components::game::Game;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Game>::new().mount_to_body();
}
