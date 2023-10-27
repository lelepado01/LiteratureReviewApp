
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::tables::memos::MemoTable;

pub fn create_memos_page(cx: Scope) -> Element {
    cx.render(rsx!(MemoTable{}))
}