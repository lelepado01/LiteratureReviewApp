
use dioxus::prelude::*;
use dioxus_sortable::{use_sorter, PartialOrdBy, SortBy, Sortable, Th};
use serde::{Deserialize, Serialize};

use crate::categories::categories_data::CategoriesData;
use crate::components::buttons::create_button_open_pdf;
use crate::common::create_search_bar;
use crate::data::loader::load_categories;

/// Our table row. Type `T`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoriesTableRow {
    pub category: String,
    pub paths: Vec<String>,
}

/// Our table columns. Type `F`. One for each field in Person.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum CategoriesTableField {
    #[default]
    Category,
    FileName,
    Open,
}

/// Specify how we sort our `Person` using `PersonField`.
impl PartialOrdBy<CategoriesTableRow> for CategoriesTableField {
    fn partial_cmp_by(&self, a: &CategoriesTableRow, b: &CategoriesTableRow) -> Option<std::cmp::Ordering> {
        // Note how it's just a passthru to `PartialOrd` for each field.
        match self {
            CategoriesTableField::Category => a.category.partial_cmp(&b.category),
            CategoriesTableField::FileName => Some(std::cmp::Ordering::Equal),
            CategoriesTableField::Open => Some(std::cmp::Ordering::Equal),
        }
    }
}

/// Specify sorting options available on a column.
impl Sortable for CategoriesTableField {
    fn sort_by(&self) -> Option<SortBy> {
        // We can choose column specifics but this is good for the minimum.
        SortBy::increasing_or_decreasing()
    }
}

pub fn CategoriesTable<'a>(cx: Scope<'a>, categories_data : CategoriesData<'a>) -> Element<'a> {

    let mut data = load_categories(); 
    data.retain(|row| row.category.to_lowercase().contains(categories_data.search_query.get()));
    categories_data.sorter.sort(data.as_mut_slice());

    cx.render(rsx!{
        div { 
            class: "mx-auto p-4 bg-gray-100 flex justify-center",
            create_search_bar(cx, categories_data.search_query)
            div { 
                class: "p-2"
            }
            div { class: "flex items-center justify-center flex-row",
                table {
                    thead {
                        tr {
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::Category, "Category" }
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::FileName, "File" }
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::Open, "" }
                        }
                    }
                    tbody {
                        for table_row in data.iter() {
                            for i in 0..table_row.paths.len() {
                                tr {
                                    td { "{table_row.category}" }
                                    td { create_reference(cx, table_row.paths[i].clone()) }
                                    create_button_open_pdf(cx, "Open".to_string(), table_row.paths[i].clone()) 
                                }
                            }
                        }
                    }
                }
            }
        }
   })
}

fn create_reference<'a>(cx: Scope<'a>, reference : String) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center justify-center",
            reference,
        }
    })
}