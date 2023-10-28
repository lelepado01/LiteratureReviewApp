
use dioxus::prelude::*;

use crate::paper_search::paper_search_results::PaperSearchResult;

#[derive(Clone, Debug, PartialEq)]
pub struct PaperSearchData<'a> {
    pub search_query: &'a UseState<String>, 
    pub search_results: &'a UseState<Vec<PaperSearchResult>>, 
    pub abstract_modal_data: &'a UseState<String>,
    pub abstract_modal_hidden: &'a UseState<bool>,
}

impl<'a> PaperSearchData<'a> {
    pub fn new(cx: Scope<'a>) -> Self { 
        let search_query: &UseState<String> = use_state(cx, || "".to_string());
        let search_results: &UseState<Vec<PaperSearchResult>> = use_state(cx, || {vec![]});
        let abstract_modal_data: &UseState<String> = use_state(cx, || "".to_string());
        let abstract_modal_hidden: &UseState<bool> = use_state(cx, || true);
    
        PaperSearchData { search_query, search_results, abstract_modal_data, abstract_modal_hidden }
    }
}