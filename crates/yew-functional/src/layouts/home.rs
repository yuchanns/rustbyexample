use crate::components::head::Head;
use crate::components::layout::Layout;
use crate::components::top::Top;
use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Head />
            <Layout>
                <Top />
            </Layout>
        </>
    }
}
