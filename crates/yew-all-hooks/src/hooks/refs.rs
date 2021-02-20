use yew::prelude::*;
use yew_functional::*;

#[function_component(UseRef)]
pub fn ref_hook() -> Html {
    let (message, set_message) = use_state(|| "".to_string());
    let message_count = use_ref(|| 0);

    let onclick = Callback::from(move |_e| {
        let window = yew::utils::window();

        if *message_count.borrow_mut() > 3 {
            window.alert_with_message("Message limit reached");
        } else {
            *message_count.borrow_mut() += 1;
            window.alert_with_message("Message sent");
        }
    });

    let onchange = Callback::from(move |e| {
        if let ChangeData::Value(value) = e {
            set_message(value)
        }
    });

    html! {
        <div>
            <input onchange=onchange value=message />
            <button onclick=onclick>{ "Send" }</button>
        </div>
    }
}
