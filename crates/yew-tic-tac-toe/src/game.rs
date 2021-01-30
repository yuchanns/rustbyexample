use crate::board::Board;
use crate::utils::calculate_winner;
use yew::prelude::*;

#[derive(Copy, Clone)]
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
        self.history.truncate(step + 1);
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
          <>
            <div class="head">
              <a href="https://github.com/yuchanns/rustbyexample/tree/main/crates/yew-tic-tac-toe" target="_blank">
                <svg aria-labelledby="simpleicons-github-dark-icon" lang="" role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="head-icon">
                  <title id="simpleicons-github-dark-icon" lang="en">{"GitHub Dark icon"}</title>
                  <path fill="#7F8C8D" d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"></path>
                </svg>
              </a>
            </div>
            <div class="game">
              <div class="game-board">
                <Board squares={current.squares} onclick={self.link.callback(|i: usize| Msg::HandleClick(i))} />
              </div>
              <div class="game-info">
                <div>{status}</div>
                <ol>{for history.iter().enumerate().map(|e| self.display_moves(e))}</ol>
              </div>
            </div>
          </>
        }
    }
}
