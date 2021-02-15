use std::rc::Rc;
use yew::prelude::*;
use yew_functional::{function_component, use_state};

#[function_component(Counter)]
pub fn state() -> Html {
    let (counter, set_counter) = use_state(|| 0);
    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    html! {
        <div>
            <button onclick=onclick>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { counter }
            </p>
        </div>
    }
}
