use yew::prelude::*;
use yew_functional::*;

use crate::components::icons::{
    feather_twitter::FeatherTwitter, ri_bookmark::RiBookmark, ri_sticky_notes::RiStickyNotes,
    util_github_alt::UtilGithubAlt,
};

use crate::components::toggle_theme::ToggleTheme;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let (is_dark, set_is_dark) = {
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
        };
        use_state(move || is_perfered_dark)
    };
    let set_is_dark_cb = Callback::from(move |is_dark: bool| set_is_dark(is_dark));
    html! {
        <header class="header">
            <nav class="nav">
                <a href="/" class="w-10 h-10 absolute lg:fixed m-6 select-none outline-none" focusable="false">
                    {if *is_dark {
                    html!{<img src="/rust-moon.svg" alt="logo" />}
                    } else {
                    html!{<img src="/rust.svg" alt="logo" />}
                    }}
                </a>
                <div class="spacer"></div>
                <div class="right">
                <a href="https://yuchanns.xyz">{"Blog"}</a>
                <a href="https://github.com/yuchanns?tab=repositories&type=source">{"Projects"}</a>
                <a title="Bookmarks"><RiBookmark /></a>
                <a title="Bookmarks"><RiStickyNotes /></a>
                <a href="https://twitter.com/airamusume" title="twitter" class="hidden md:block"><FeatherTwitter /></a>
                <a href="https://github.com/yuchanns" title="github" class="hidden md:block"><UtilGithubAlt /></a>
                <ToggleTheme is_dark=is_dark.clone() set_is_dark=set_is_dark_cb />
                </div>
            </nav>
        </header>
    }
}
