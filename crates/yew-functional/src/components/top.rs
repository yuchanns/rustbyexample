use crate::components::article::Article;
use yew::prelude::*;
use yew_functional::function_component;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
       <div>
        <div class="about-container head-block body-border">
          <div class="about-box">
            <div class="about-title">
              <div class="title-text">
                {"readme"}<span class="text-gray-light">{".md"}</span>
              </div>
            </div>
            <Article class="about-content markdown-body mask" style="overflow-y: hidden" inner_html=r#"<p>Hi there~I'm <a href="https://github.com/yuchanns">@yuchanns</a>", having a fregquently used Chinese nickname 科学搜查官 which is a profession of <a href="https://w.atwiki.jp/aniwotawiki/pages/31294.html">Ema Skye</a> from the video game Phoenix Wright.</p>
    <p>And my avatar was illustrated by a friend, a gopher drinking coffe and coding, which acts as one sticker of Bronya from mobile game Honkai3.</p>
                    <p>I'm a backend developer using Golang as the main language. I also have some appreciation on Rust and Lisp.</p>
                    <p>An ambisious target is to hold a setup of Computer Science from the bottom up. I wish to be a Lifelong CS user.</p>
                    <p>You can touch me with <a href="mailto:airamusume@gmail.com">email</a>.</p>"# />
          </div>
        </div>
      </div>
    }
}
