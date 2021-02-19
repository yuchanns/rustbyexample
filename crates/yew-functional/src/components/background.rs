use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Background)]
pub fn background() -> Html {
    html! {
        <>
          <div class="body"></div>
        </>
    }
}
