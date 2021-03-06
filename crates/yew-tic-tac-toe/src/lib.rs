#![recursion_limit = "256"]

mod board;
mod game;
mod icons;
mod square;
mod utils;

use crate::game::Game;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Game>::new().mount_to_body();
}
