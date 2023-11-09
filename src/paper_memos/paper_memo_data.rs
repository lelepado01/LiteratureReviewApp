
use dioxus::prelude::*;
use dioxus_sortable::{UseSorter, use_sorter};
use serde::{Serialize, Deserialize};

use super::paper_memo_table::PaperMemoTableField;

pub struct PaperMemoData<'a> {
    pub search_query: &'a UseState<String>,
    pub sorter: UseSorter<'a, PaperMemoTableField>,
}

impl PaperMemoData<'_> {
    pub fn new(cx: Scope) -> PaperMemoData {
        let search_query = use_state(cx, || "".to_string());
        let sorter = use_sorter::<PaperMemoTableField>(cx);
        PaperMemoData {
            search_query,
            sorter,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaperMemo {
    pub paper_name: String,
    pub memo_data: String,
}