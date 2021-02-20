use std::rc::Rc;
use yew::prelude::*;
use yew_functional::*;

#[function_component(UseEffect)]
pub fn effect() -> Html {
    let (counter, set_counter) = use_state(|| 0);

    {
        let counter = counter.clone();
        use_effect(move || {
            // Make a call to DOM API after component is rendered
            yew::utils::document().set_title(&format!("You clicked {} times", counter));

            // Perform the cleanup
            || yew::utils::document().set_title("You clicked 0 times")
        });
    }
    let onclick = {
        let counter = Rc::clone(&counter);
        Callback::from(move |_| set_counter(*counter + 1))
    };

    html! {
        <button onclick=onclick>{ format!("Increment to {}", counter) }</button>
    }
}
