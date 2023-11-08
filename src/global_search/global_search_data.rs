
use dioxus::prelude::*;
use futures::stream::FuturesUnordered;

use super::global_search_results::GlobalSearchResult;

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSearchData<'a> {
    pub search_query: &'a UseState<String>,
    pub search_results: &'a UseState<Vec<GlobalSearchResult>>,
    pub search_results_async: &'a UseState<FuturesUnordered<GlobalSearchResult>>,
}

impl <'a> GlobalSearchData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        Self {
            search_query: use_state(cx, || "".to_string()),
            search_results: use_state(cx, || { Vec::new() }),
            search_results_async: use_state(cx, || { FuturesUnordered::new() }),
        }
    }
}