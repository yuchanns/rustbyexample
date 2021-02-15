mod counter;

use counter::Counter;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Counter>::new().mount_to_body();
}
