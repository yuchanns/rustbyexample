use yew::prelude::*;
use yew_functional::function_component;

#[function_component(User)]
pub fn user() -> Html {
    html! {
        <>
            <div class="clearfix card-user">
              <div class="card-avatar-container">
                <img class="card-avatar" width="260" height="260" src="https://yuchanns.xyz/yuchanns.jpg" alt="yuchanns" />
              </div>
              <div class="card-name-container">
                <h1 class="card-names">
                  <span class="card-name">{"科学搜查官"}</span>
                  <span class="card-nickname">{"yuchanns"}</span>
                </h1>
              </div>
            </div>
            <div class="card-note">
              <div>{"magnum opus. Gopher / Rustacean"}</div>
            </div>
        </>
    }
}
