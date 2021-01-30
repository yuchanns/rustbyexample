use yew::prelude::*;

pub struct Yew {}

impl Component for Yew {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        return Self {};
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 75 82" fill="none" class="head-icon">
            <circle cx="38" cy="40" r="25" fill="#FFEFB8"/>
            <path d="M38.2373 41.0339L14 14" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
            <path d="M38.2373 41.0339L62.4746 14" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
            <path d="M38.2373 41.0339L38.2373 69" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
            <circle cx="38" cy="41" r="7" fill="#FFD707" stroke="#444444" stroke-width="4"/>
            <script xmlns=""/>
          </svg>
        }
    }
}
