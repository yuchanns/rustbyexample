use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<()>,
    pub value: &'static str,
}

pub enum Msg {
    HandleClick,
}

pub struct Square {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Square {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleClick => self.props.onclick.emit(()),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
          <button class="square" onclick={self.link.callback(|_| Msg::HandleClick)}>
            {self.props.value}
          </button>
        }
    }
}
