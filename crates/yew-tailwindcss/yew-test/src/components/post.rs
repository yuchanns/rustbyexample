use yew::prelude::*;
use yew::web_sys::Element;
use yew_functional::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub inner_html: String,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let node_ref = NodeRef::default();
    {
        let inner_html = props.inner_html.clone();
        let node_ref = node_ref.clone();
        use_effect(move || {
            let el = node_ref.cast::<Element>().unwrap();
            el.set_inner_html(inner_html.as_str());
            || {}
        });
    }
    html! {
      <>
        <div class="prose m-auto mb-8">
          <h1 class="mb-0">{"Yew Tailwindcss"}</h1>
        </div>
        <div class="prose m-auto" ref=node_ref.clone() />
      </>
    }
}
