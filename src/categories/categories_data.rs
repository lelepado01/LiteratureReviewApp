
use serde::{Serialize, Deserialize};

use dioxus::prelude::*;
use dioxus_sortable::{UseSorter, use_sorter};

use crate::categories::categories_table::CategoriesTableField;

#[derive(Clone, Copy)]
pub struct CategoriesData<'a> {
    pub search_query : &'a UseState<String>,
    pub sorter: UseSorter<'a, CategoriesTableField>,
    pub color_picker_modal_color : &'a UseState<String>,
    pub color_picker_row : &'a UseState<Option<usize>>,
    pub category_name_temp : &'a UseState<String>,
}

impl <'a> CategoriesData<'a> {
    pub fn new(cx: Scope<'a>) -> Self {
        Self {
            search_query: use_state(cx, || "".to_string()),
            sorter: use_sorter::<CategoriesTableField>(cx),
            color_picker_modal_color: use_state(cx, || "".to_string()),
            color_picker_row: use_state(cx, || None),
            category_name_temp: use_state(cx, || "".to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CategoryTag {
    pub label: String,
    pub color: String,
}