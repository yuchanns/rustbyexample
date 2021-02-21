use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[derive(Clone, PartialEq, Properties)]
struct Props {
    onclick: Callback<usize>,
}

#[function_component(ButtonOuter)]
pub fn button_outer() -> Html {
    let (number, set_number) = use_state(|| 0usize);
    let onclick = Callback::from(move |i: usize| set_number(i));
    html! {
        <div>
            <p>{number}</p>
            <ButtonGroup onclick=onclick />
        </div>
    }
}

#[function_component(ButtonGroup)]
fn button_group(props: &Props) -> Html {
    let render_button = |i: usize| {
        let onclick = props.onclick.clone();
        html! {
          <button onclick={ Callback::from(move |_| onclick.emit(i)) }>
            {i}
          </button>
        }
    };
    html! {
      <>
        {render_button(1)}
        {render_button(2)}
      </>
    }
}
