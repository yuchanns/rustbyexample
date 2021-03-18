use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_functional::*;

use crate::components::icons::ri_sun::RiSun;
use std::rc::Rc;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub is_dark: Rc<bool>,
    pub set_is_dark: Callback<bool>,
}

fn set_class(is_dark: bool) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element = document
        .document_element()
        .expect("should hav a element on document");
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    let class_list = element.class_list();
    if is_dark {
        class_list.add(&arr).expect("should add dark class success");
    } else {
        class_list
            .remove(&arr)
            .expect("should remove dark class success");
    }
}

#[function_component(ToggleTheme)]
pub fn toggle_theme(props: &Props) -> Html {
    use_effect({
        let is_dark = props.is_dark.clone();
        move || {
            set_class(*is_dark);
            || {}
        }
    });
    let onclick = {
        let (is_dark, set_is_dark) = (props.is_dark.clone(), props.set_is_dark.clone());
        Callback::from(move |_| {
            set_is_dark.emit(!*is_dark);
            set_class(!*is_dark);
        })
    };
    html! {
      <a class="select-none" title="Toggle Color Scheme" onclick=onclick>
        <RiSun />
      </a>
    }
}
