use yew::prelude::*;
use yew_functional::*;

use crate::components::icons::{
    feather_twitter::FeatherTwitter, ri_bookmark::RiBookmark, ri_sticky_notes::RiStickyNotes,
    util_github_alt::UtilGithubAlt,
};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="header">
            <nav class="nav">
                <a href="/" class="w-10 h-10 absolute lg:fixed m-6 select-none outline-none" focusable="false">
                    <img src="/rust.svg" alt="logo" />
                </a>
                <div class="spacer"></div>
                <div class="right">
                <a href="https://yuchanns.xyz">{"Blog"}</a>
                <a href="https://github.com/yuchanns?tab=repositories&type=source">{"Projects"}</a>
                <a title="Bookmarks"><RiBookmark /></a>
                <a title="Bookmarks"><RiStickyNotes /></a>
                <a href="https://twitter.com/airamusume" title="twitter" class="hidden md:block"><FeatherTwitter /></a>
                <a href="https://github.com/yuchanns" title="github" class="hidden md:block"><UtilGithubAlt /></a>
                </div>
            </nav>
        </header>
    }
}
