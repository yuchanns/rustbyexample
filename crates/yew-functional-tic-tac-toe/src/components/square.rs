use yew::prelude::*;
use yew_functional::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<()>,
    pub value: &'static str,
}

#[function_component(Square)]
pub fn square(props: &Props) -> Html {
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |_| onclick.emit(()))
    };
    html! {
        <button class="square" onclick=onclick>
            {props.value}
        </button>
    }
}
