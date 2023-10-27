
use dioxus::prelude::*;

use super::global_search_results::GlobalSearchResult;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSearchData<'a> {
    pub search_query: &'a UseState<String>,
    pub search_results: &'a UseState<Vec<GlobalSearchResult>>,
}

impl <'a> GlobalSearchData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        Self {
            search_query: use_state(cx, || "".to_string()),
            search_results: use_state(cx, || vec![]),
        }
    }
}