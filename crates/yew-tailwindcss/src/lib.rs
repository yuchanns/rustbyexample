use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_functional::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
      <div class="container mx-auto px-4">
        <figure class="md:flex bg-gray-100 rounded-xl p-8 md:p-0">
          <img class="w-32 h-32 md:w-48 md:h-auto md:rounded-none rounded-full mx-auto" src="/yuchanns.jpeg" alt="" width="384" height="512" />
          <div class="pt-6 md:p-8 text-center md:text-left space-y-4">
            <blockquote>
              <p class="text-lg font-semibold">
                {"“I'd like to try Tailwind CSS on my Rust Wasm projects.”"}
              </p>
            </blockquote>
            <figcaption class="font-medium">
              <div class="text-cyan-600">
                {"yuchanns"}
              </div>
              <div class="text-gray-500">
                {"Backend Developer"}
              </div>
            </figcaption>
          </div>
        </figure>
      </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<HelloWorld>::new().mount_to_body();
}
