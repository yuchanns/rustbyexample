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

#[function_component(ToggleTheme)]
pub fn toggle_theme(props: &Props) -> Html {
    let window = web_sys::window().expect("no global `window` exists");
    let mut is_perfered_dark = false;
    match window.match_media("(prefers-color-scheme: dark)") {
        Ok(option_media_query_list) => match option_media_query_list {
            Some(media_query_list) => {
                is_perfered_dark = media_query_list.matches();
            }
            None => {}
        },
        Err(_) => {}
    }
    let document = window.document().expect("should have a document on window");
    let element = document
        .document_element()
        .expect("should hav a element on document");
    let arr = js_sys::Array::new_with_length(1);
    arr.set(0, JsValue::from_str("dark"));
    let class_list = element.class_list();
    {
        let class_list = class_list.clone();
        let arr = arr.clone();
        use_effect(move || {
            if is_perfered_dark {
                class_list.add(&arr).expect("should add dark class success");
            } else {
                class_list
                    .remove(&arr)
                    .expect("should remove dark class success");
            }
            || {}
        });
    }
    let onclick = {
        let (is_dark, set_is_dark) = (props.is_dark.clone(), props.set_is_dark.clone());
        if *is_dark {
            class_list
                .remove(&arr)
                .expect("should remove dark class success");
        } else {
            class_list.add(&arr).expect("should add dark class success");
        }
        Callback::from(move |_| {
            set_is_dark.emit(!*is_dark);
            if *is_dark {
                class_list
                    .remove(&arr)
                    .expect("should remove dark class success");
            } else {
                class_list.add(&arr).expect("should add dark class success");
            }
        })
    };
    html! {
      <a class="select-none" title="Toggle Color Scheme" onclick=onclick>
        <RiSun />
      </a>
    }
}
