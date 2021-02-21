use crate::components::square::Square;
use crate::utils::calculator::BOARD;
use yew::prelude::*;
use yew_functional::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub squares: [&'static str; 9],
    pub onclick: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let render_square = |i: usize| {
        let onclick = props.onclick.clone();
        let value = props.squares[i];
        html! {<Square value={value} onclick=Callback::from(move |_| onclick.emit(i)) />}
    };
    let render_row = |item: &[usize; 3]| {
        html! {
            <div class="board-row">
            {item.iter().map(|i: &usize| render_square(*i)).collect::<Html>()}
            </div>
        }
    };
    html! {
      <div>
        {BOARD.iter().map(render_row).collect::<Html>()}
      </div>
    }
}
