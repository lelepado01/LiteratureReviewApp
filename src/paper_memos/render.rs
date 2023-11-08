use dioxus::prelude::*;

pub fn create_paper_memos_page(cx : Scope) -> Element {
    cx.render(rsx! {
        div{
            h1 { "Paper Content" }
            p { "This is the paper content page"}
        }
    })
}