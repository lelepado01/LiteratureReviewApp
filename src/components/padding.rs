
use dioxus::prelude::*;

pub fn create_padding_block(cx : Scope) -> Element {
    cx.render(rsx!(
        div{
            class: "p-1"
        }
    ))
}