use crate::components::background::Background;
use crate::components::card::Card;
use yew::prelude::*;
use yew_functional::function_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
       <div class="container body-container">
        <Background />
        <div class="head-inner">
          <Card />
          <div class="block head-block-nav main-container">
          {props.children.clone()}
          </div>
        </div>
      </div>
    }
}
