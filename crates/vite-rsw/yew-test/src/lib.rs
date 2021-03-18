use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_functional::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <div>{"hello world"}</div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<HelloWorld>::new().mount_to_body();
}
