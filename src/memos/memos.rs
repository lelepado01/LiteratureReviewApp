
use dioxus::prelude::*;

use crate::memos::memos_table::MemoTable;

pub fn create_memos_page(cx: Scope) -> Element {
    cx.render(rsx!(MemoTable{}))
}