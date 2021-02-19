use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Head)]
pub fn head() -> Html {
    html! {
       <div class="head">
        <div class="container">
          <div class="head-inner">
            <div class="head-block head-block-user"></div>
            <div class="head-block head-block-nav">
              <div class="nav-container">
                <nav class="nav-body">
                    <a class="nav-item">{"Home"}</a>
                    <a class="nav-item">{"Go"}</a>
                    <a class="nav-item">{"Rust"}</a>
                </nav>
              </div>
            </div>
          </div>
        </div>
       </div>
    }
}
