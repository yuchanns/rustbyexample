use crate::board::Board;
use crate::utils::calculate_winner;
use yew::prelude::*;

struct History {
    squares: [&'static str; 9],
}

pub struct Game {
    link: ComponentLink<Self>,
    history: Vec<History>,
    step_number: usize,
    x_is_next: bool,
}

pub enum Msg {
    HandleClick(usize),
    JumpTo(usize),
}

impl Game {
    fn handle_click(&mut self, i: usize) {
        let history_len = &self.history[0..(self.step_number + 1)].len();
        let squares = &self.history[history_len - 1].squares;
        if calculate_winner(*squares) != "" || squares[i] != "" {
            return;
        }
        let mut squares_new = *squares;
        squares_new[i] = if self.x_is_next { "X" } else { "O" };
        self.history.push(History {
            squares: squares_new,
        });
        self.step_number = *history_len;
        self.x_is_next = !self.x_is_next;
    }

    fn jump_to(&mut self, step: usize) {
        if step == 0 {
            self.history = vec![History { squares: [""; 9] }];
        }
        self.step_number = step;
        self.x_is_next = (step % 2) == 0;
    }

    fn display_moves(&self, (index, _): (usize, &History)) -> Html {
        let desc = if index == 0 {
            format!("Go to game start")
        } else {
            format!("Go to move #{}", index)
        };
        html! {
           <li key={index}>
             <button onclick={self.link.callback(move |_| Msg::JumpTo(index))}>{desc}</button>
           </li>
        }
    }
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            history: vec![History { squares: [""; 9] }],
            step_number: 0,
            x_is_next: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleClick(i) => {
                self.handle_click(i);
            }
            Msg::JumpTo(i) => {
                self.jump_to(i);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let history = &self.history;
        let current: &History = &history[self.step_number];
        let winner = calculate_winner(current.squares);
        let status = if winner != "" {
            format!("Winner: {}", winner)
        } else {
            format!("Next player: {}", if self.x_is_next { "X" } else { "O" })
        };
        html! {
          <div class="game">
            <div class="game-board">
              <Board squares={current.squares} onclick={self.link.callback(|i: usize| Msg::HandleClick(i))} />
            </div>
            <div class="game-info">
              <div>{status}</div>
              <ol>{for history.iter().enumerate().map(|e| self.display_moves(e))}</ol>
            </div>
          </div>
        }
    }
}
