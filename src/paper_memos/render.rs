use dioxus::prelude::*;

use crate::paper_memos::paper_memo_data::PaperMemoData;
use crate::paper_memos::paper_memo_table::PaperMemoTable;

pub fn create_paper_memos_page<'a>(cx : Scope<'a>, papermemo_data : PaperMemoData<'a>) -> Element<'a> {
    cx.render(rsx! {
        PaperMemoTable(cx, papermemo_data)
    })
}