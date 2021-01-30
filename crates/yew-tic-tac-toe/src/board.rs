use crate::square::Square;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub squares: [&'static str; 9],
    pub onclick: Callback<usize>,
}

pub enum Msg {
    HandleClick(usize),
}

pub struct Board {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Board {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Self { link, props };
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleClick(i) => self.props.onclick.emit(i),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let render_square = |i: usize| {
            html! {<Square value={self.props.squares[i]} onclick={self.link.callback(move |_| Msg::HandleClick(i))} />}
        };
        html! {
          <div>
            <div class="board-row">
              {render_square(0)}
              {render_square(1)}
              {render_square(2)}
            </div>
            <div class="board-row">
              {render_square(3)}
              {render_square(4)}
              {render_square(5)}
            </div>
            <div class="board-row">
              {render_square(6)}
              {render_square(7)}
              {render_square(8)}
            </div>
          </div>
        }
    }
}
