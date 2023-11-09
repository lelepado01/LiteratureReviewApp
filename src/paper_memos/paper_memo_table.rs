
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};

use crate::data::loader::{LoaderResult, load_paper_memos, load_pdf_details};
use crate::paper_memos::paper_memo_data::PaperMemoData;
use crate::common::create_search_bar;

use super::paper_memo_data::PaperMemo;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum PaperMemoTableField {
    #[default]
    FileName,
    MemoData,
}

impl PartialOrdBy<PaperMemo> for PaperMemoTableField {
    fn partial_cmp_by(&self, a: &PaperMemo, b: &PaperMemo) -> Option<std::cmp::Ordering> {
        match self {
            PaperMemoTableField::FileName => a.paper_name.partial_cmp(&b.paper_name),
            PaperMemoTableField::MemoData => a.memo_data.partial_cmp(&b.memo_data),
        }
    }
}

impl Sortable for PaperMemoTableField {
    fn sort_by(&self) -> Option<SortBy> {
        SortBy::increasing_or_decreasing()
    }
}

pub fn PaperMemoTable<'a>(cx: Scope<'a>, papermemo_data : PaperMemoData<'a>) -> Element<'a> {
    let data = load_paper_memos(papermemo_data.search_query.get());
        
    match data {
        LoaderResult::Ok(mut data) => {
            for row in data.iter() {
                load_pdf_details(&row.paper_name); 
            }
            papermemo_data.sorter.sort(data.as_mut_slice());
            cx.render(rsx!{
                div { 
                    class: "mx-auto p-4 bg-gray-100 flex justify-center",
                    create_search_bar(cx, papermemo_data.search_query)
                    div { 
                        class: "p-2"
                    }
                    div { class: "flex items-center justify-center flex-row",
                        table {
                            thead {
                                tr {
                                    Th { sorter: papermemo_data.sorter, field: PaperMemoTableField::FileName, "Name" }
                                    Th { sorter: papermemo_data.sorter, field: PaperMemoTableField::MemoData, "Memo" }
                                }
                            }
                            tbody {
                                for row in data.iter() {
                                    tr {
                                        td { class: "border px-4 py-2", row.paper_name.clone() }
                                        td { class: "border px-4 py-2", row.memo_data.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
           })
        },
        _ => None
    }
}
