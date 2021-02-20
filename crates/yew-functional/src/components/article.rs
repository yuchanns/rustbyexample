use yew::prelude::*;
use yew::web_sys::Element;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub inner_html: String,
    pub class: String,
    pub style: String,
}

pub struct Article {
    props: Props,
    node_ref: NodeRef,
}

impl Component for Article {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        // create the parent element and store a reference to it
        html! {
            <article ref=self.node_ref.clone() class=self.props.class.clone() style=self.props.style.clone() />
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&self.props.inner_html);
    }
}
