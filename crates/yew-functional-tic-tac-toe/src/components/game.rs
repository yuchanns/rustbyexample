use crate::components::board::Board;
use crate::components::icons::octocat::OctoCat;
use crate::components::icons::rust::Rust;
use crate::components::icons::yewstack::YewStack;
use crate::utils::calculator::calculate_winner;
use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[derive(Copy, Clone)]
struct History {
    squares: [&'static str; 9],
}

#[function_component(Game)]
pub fn game() -> Html {
    let (history, set_history) = use_state(|| vec![History { squares: [""; 9] }]);
    let (step_number, set_step_number) = use_state(|| 0);
    let (x_is_next, set_x_is_next) = use_state(|| true);

    let current: &History = &history[*step_number];
    let winner = calculate_winner(current.squares);
    let status = if winner != "" {
        format!("Winner: {}", winner)
    } else {
        format!("Next player: {}", if *x_is_next { "X" } else { "O" })
    };

    let render_board = {
        let history = Rc::clone(&history);
        let set_history = Rc::clone(&set_history);
        let step_number = Rc::clone(&step_number);
        let set_step_number = Rc::clone(&set_step_number);
        let x_is_next = Rc::clone(&x_is_next);
        let set_x_is_next = Rc::clone(&set_x_is_next);
        let onclick = {
            Callback::from(move |i: usize| {
                let history_len = history[0..(*step_number + 1)].len();
                let squares = &history[history_len - 1].squares;
                if calculate_winner(*squares) != "" || squares[i] != "" {
                    return;
                }
                let mut squares_new = *squares;
                squares_new[i] = if *x_is_next { "X" } else { "O" };
                let mut new_history: Vec<History> = vec![];
                new_history.extend(&*history);
                new_history.push(History {
                    squares: squares_new,
                });
                set_history(new_history);
                set_step_number(history_len);
                set_x_is_next(if *x_is_next { false } else { true });
            })
        };
        || {
            html! {
                <Board squares={current.squares} onclick=onclick />
            }
        }
    };

    let display_moves = |(step, _): (usize, &History)| {
        let desc = if step == 0 {
            format!("Go to game start")
        } else {
            format!("Go to move #{}", step)
        };
        let onclick = {
            let set_step_number = Rc::clone(&set_step_number);
            let set_x_is_next = Rc::clone(&set_x_is_next);
            let history = Rc::clone(&history);
            let set_history = Rc::clone(&set_history);
            Callback::from(move |_| {
                set_step_number(step);
                set_x_is_next((step % 2) == 0);
                let mut new_history: Vec<History> = vec![];
                new_history.extend(&*history);
                new_history.truncate(step + 1);
                set_history(new_history);
            })
        };
        html! {
           <li key={step}>
             <button onclick=onclick>{desc}</button>
           </li>
        }
    };

    html! {
      <>
        <div class="head">
          <a href="https://github.com/yuchanns/rustbyexample/tree/main/crates/yew-functional-tic-tac-toe" target="_blank" class="head-icon-link">
            <OctoCat />
          </a>
          <a href="https://github.com/yewstack/yew" target="_blank" class="head-icon-link">
            <YewStack />
          </a>
          <a href="https://github.com/rust-lang/rust" target="_blank" class="head-icon-link">
            <Rust />
          </a>
        </div>
        <div class="game">
          <div class="game-board">
            {render_board()}
          </div>
          <div class="game-info">
            <div>{status}</div>
            <ol>{for history.iter().enumerate().map(display_moves)}</ol>
          </div>
        </div>
      </>
    }
}
