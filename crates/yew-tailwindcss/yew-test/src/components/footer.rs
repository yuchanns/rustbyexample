use yew::prelude::*;
use yew_functional::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
      <div class="mt-10 mb-6 prose m-auto opacity-50 flex">
        <span class="text-sm"><a target="_blank" href="https://creativecommons.org/licenses/by-nc/4.0/" style="color:inherit">{"CC BY-NC 4.0"}</a>{" 2021 © yuchanns"}</span>
        <div class="flex-auto" />
      </div>
    }
}
