
use dioxus::prelude::*;
use dioxus_sortable::{UseSorter, use_sorter};

use crate::categories::categories_table::CategoriesTableField;

pub struct CategoriesData<'a> {
    pub search_query : &'a UseState<String>,
    pub sorter: UseSorter<'a, CategoriesTableField>,

}

impl <'a> CategoriesData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        Self {
            search_query: use_state(cx, || "".to_string()),
            sorter: use_sorter::<CategoriesTableField>(cx),
        }
    }
}

struct CategoryTag {
    label: String,
    color: String,
}