use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[function_component(UseReducer)]
pub fn reducer() -> Html {
    /// reducer's Action
    enum Action {
        Double,
        Square,
    }

    /// reducer's State
    struct CounterState {
        counter: i32,
    }

    let (
        counter, // the state
        // function to update the state
        // as the same suggests, it dispatches the values to the reducer function
        dispatch,
    ) = use_reducer(
        // the reducer function
        |prev: Rc<CounterState>, action: Action| CounterState {
            counter: match action {
                Action::Double => prev.counter * 2,
                Action::Square => prev.counter * prev.counter,
            },
        },
        // initial state
        CounterState { counter: 1 },
    );

    let double_onclick = {
        let dispatch = Rc::clone(&dispatch);
        Callback::from(move |_| dispatch(Action::Double))
    };
    let square_onclick = Callback::from(move |_| dispatch(Action::Square));

    html! {
        <>
            <div id="result">{ counter.counter }</div>

            <button onclick=double_onclick>{ "Double" }</button>
            <button onclick=square_onclick>{ "Square" }</button>
        </>
    }
}
