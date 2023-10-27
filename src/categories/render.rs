
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::categories::categories_table::CategoriesTable;
use crate::categories::categories_data::CategoriesData;

pub fn create_categories_page<'a>(cx: Scope<'a>, categories_data : CategoriesData<'a>) -> Element<'a> {
    cx.render(rsx!(CategoriesTable(cx, categories_data)))
}