
use dioxus::prelude::*;
use dioxus_sortable::{UseSorter, use_sorter};

use super::export_pdf_table::ExportPDFTableField;

#[derive(Clone, Debug, PartialEq)]
pub struct CitationData {
    pub author: String,
    pub title: String,
    pub year: String,
    pub publisher: String,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ExportData<'a> {
    pub sorter: UseSorter<'a, ExportPDFTableField>,
    pub search_query: &'a UseState<String>,
    pub citation_data: &'a UseRef<Vec<CitationData>>,
}

impl<'a> ExportData<'a> {
    pub fn new(cx : Scope<'a>) -> Self {
        ExportData {
            sorter: use_sorter::<ExportPDFTableField>(cx),
            search_query: use_state(cx, || {"".to_string()}),
            citation_data: use_ref(cx, || {vec![]}),
        }
    }

    pub fn add_citation_data(&self, citation_data : CitationData) {
        self.citation_data.with_mut(|data| {
            data.push(citation_data);
        });
    }

    pub fn remove_citation_data(&self, citation_data : CitationData) {
        self.citation_data.with_mut(|data| {
            data.retain(|cit| cit != &citation_data);
        });
    }
}