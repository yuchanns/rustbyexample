mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::layout::Layout;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Layout>::new().mount_to_body();
}
