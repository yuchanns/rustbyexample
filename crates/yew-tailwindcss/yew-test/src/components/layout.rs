use yew::prelude::*;
use yew_functional::*;

use crate::components::{footer::Footer, navbar::Navbar, post::Post};

#[function_component(Layout)]
pub fn layout() -> Html {
    html! {
      <>
        <Navbar />
        <main class="px-7 py-10">
            <Post inner_html=r#"
<p>Hi there~I'm <a href="https://github.com/yuchanns">@yuchanns</a>, having a fregquently used Chinese nickname 科学搜查官 which is a profession of <a href="https://w.atwiki.jp/aniwotawiki/pages/31294.html" rel="nofollow">Ema Skye</a> from the video game Phoenix Wright.</p>
<p>And my avatar was illustrated by a friend, a gopher drinking coffe and coding,  which acts as one sticker of Bronya from mobile game Honkai3.</p>
<p><a target="_blank" rel="noopener noreferrer" href="/yuchanns/yuchanns/blob/master/readme.png"><img src="https://yuchanns.xyz/static/6d1e0408e2dc22f49ffd5b7ee2c7ec51/c83ae/readme.png" alt="" style="max-width:100%;"></a></p>
<p>I'm a backend developer using Golang as the main language. I also have some appreciation on Rust and Lisp.</p>
<p>An ambisious target is to hold a setup of Computer Science from the bottom up. I wish to be a <strong>Lifelong</strong> CS user.</p>
<p>You can touch me with <a href="mailto:airamusume@gmail.com">email</a>.</p>
            "#.to_string() />
            <Footer />
        </main>
      </>
    }
}
