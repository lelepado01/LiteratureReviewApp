
use dioxus::prelude::*;
use dioxus_sortable::{use_sorter, UseSorter};

use crate::dashboard::dashboard_table::DashboardTableField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DashboardData<'a> {
    pub search_query: &'a UseState<String>,
    pub sorter: UseSorter<'a, DashboardTableField>,
}

impl<'a> DashboardData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        let search_query: &UseState<String> = use_state(cx, || "".to_string());
        let sorter = use_sorter::<DashboardTableField>(cx);
        DashboardData { search_query, sorter}
    }
}