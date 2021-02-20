use crate::hooks::effect::UseEffect;
use crate::hooks::reducer::UseReducer;
use crate::hooks::refs::UseRef;
use crate::hooks::state::UseState;
use yew::prelude::*;
use yew_functional::*;

#[function_component(AppStartup)]
pub fn app() -> Html {
    html! {
        <div>
            <h3>{"use_state"}</h3>
            <UseState />
            <h3>{"use_ref"}</h3>
            <UseRef />
            <h3>{"use_reducer"}</h3>
            <UseReducer />
            <h3>{"use_effect"}</h3>
            <UseEffect />
        </div>
    }
}
