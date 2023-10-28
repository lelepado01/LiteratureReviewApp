
use dioxus::prelude::*;
use dioxus_sortable::{UseSorter, use_sorter};

use super::export_pdf_table::ExportPDFTableField;

#[derive(Clone, Debug, PartialEq)]
pub struct CitationData {
    pub category: String,
    pub author: String,
    pub title: String,
    pub year: String,
    pub publisher: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ExportData<'a> {
    pub sorter: UseSorter<'a, ExportPDFTableField>,
    pub search_query: &'a UseState<String>,
    pub citation_data: &'a UseState<Vec<CitationData>>,
}

impl<'a> ExportData<'a> {
    pub fn new(cx : Scope<'a>) -> Self {
        ExportData {
            sorter: use_sorter::<ExportPDFTableField>(cx),
            search_query: use_state(cx, || {"".to_string()}),
            citation_data: use_state(cx, || {vec![]}),
        }
    }
}