
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};
use serde::{Deserialize, Serialize};

use crate::categories::categories_data::CategoriesData;
use crate::components::color_picker_modal::create_color_picker_modal;
use crate::common::create_search_bar;
use crate::data::loader::load_categories_data;
use crate::data::updater::delete_category_data;
use crate::components::badges::create_category_badge;
use crate::categories::categories_data::CategoryTag;

/// Our table row. Type `T`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoriesTableRow {
    pub category: String,
    pub color: String,
    pub paths: Vec<String>,
}

/// Our table columns. Type `F`. One for each field in Person.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum CategoriesTableField {
    #[default]
    Category,
    Color,
    ColorPicker,
    DeleteCategory,
}

/// Specify how we sort our `Person` using `PersonField`.
impl PartialOrdBy<CategoriesTableRow> for CategoriesTableField {
    fn partial_cmp_by(&self, a: &CategoriesTableRow, b: &CategoriesTableRow) -> Option<std::cmp::Ordering> {
        // Note how it's just a passthru to `PartialOrd` for each field.
        match self {
            CategoriesTableField::Category => a.category.partial_cmp(&b.category),
            CategoriesTableField::Color => Some(std::cmp::Ordering::Equal),
            CategoriesTableField::ColorPicker => Some(std::cmp::Ordering::Equal),
            CategoriesTableField::DeleteCategory => Some(std::cmp::Ordering::Equal),
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

    let mut data = load_categories_data(); 
    data.retain(|row| row.label.to_lowercase().contains(categories_data.search_query.get()));

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
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::Color, "Preview" }
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::ColorPicker, "Pick" }
                            Th { sorter: categories_data.sorter, field: CategoriesTableField::DeleteCategory, "Delete" }
                        }
                    }
                    tbody {
                        for (i, table_row) in data.iter().enumerate() {
                            tr {
                                td { "{table_row.label}" }
                                td { 
                                    create_category_badge(cx, CategoryTag { label: table_row.label.clone(), color: table_row.color.clone() })
                                }
                                td { 
                                    create_button_color_picker(cx, i, table_row.label.clone(), categories_data)
                                }
                                td {
                                    create_delete_category_button(cx, table_row.label.clone(), categories_data)
                                }
                            }
                        }
                    }
                }
            }
        }
   })
}

fn create_button_color_picker<'a>(cx: Scope<'a>, row : usize, category : String, categories_data : CategoriesData<'a>) -> Element<'a> {

    cx.render(rsx!(
        div{
            class: "flex flex-row items-center justify-center",
            button {
                class: "flex flex-row items-center justify-center",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {

                        // TODO: refactor with function which handles clicks in table
                        if let Some(picker) = categories_data.color_picker_row.get() {
                            if *picker == row {
                                categories_data.color_picker_row.set(None);
                            } else {
                                categories_data.color_picker_row.set(Some(row));
                            }
                        } else {
                            categories_data.color_picker_row.set(Some(row));
                        }
                    },
                    svg{
                        class: "h-8 w-8 text-black-500",
                        width: "24",
                        height: "24",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "2",
                        stroke: "currentColor",
                        fill: "none",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path {
                            stroke: "none",
                            d: "M0 0h24v24H0z"
                        }
                        line {
                            x1: "11",
                            y1: "7",
                            x2: "17",
                            y2: "13"
                        }
                        path {
                            d: "M5 19v-4l9.7 -9.7a1 1 0 0 1 1.4 0l2.6 2.6a1 1 0 0 1 0 1.4l-9.7 9.7h-4"
                        }
                    }
                } 
                
            }
            if let Some(picker) = categories_data.color_picker_row.get() {
                if *picker == row {
                    create_color_picker_modal(cx, category, categories_data.color_picker_row, categories_data.color_picker_modal_color)
                } else {
                    None
                }
            } else {
                None
            }
        }
    ))
}

fn create_delete_category_button<'a>(cx: Scope<'a>, category : String, categories_data : CategoriesData<'a>) -> Element<'a> {

    cx.render(rsx!(
        div{
            class: "flex flex-row items-center justify-center",
            button {
                class: "btn btn-primary",
                onclick: move |_| { 
                    delete_category_data(category.clone());
                    categories_data.category_name_temp.set("".to_string());
                },
                svg{
                    class: "h-8 w-8 text-black-500",
                    width: "24",
                    height: "24",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "2",
                    stroke: "currentColor",
                    fill: "none",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    polyline {
                        points: "3 6 5 6 21 6"
                    }
                    path {
                        d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                    }
                    line {
                        x1: "10",
                        y1: "11",
                        x2: "10",
                        y2: "17"
                    }
                    line {
                        x1: "14",
                        y1: "11",
                        x2: "14",
                        y2: "17"
                    }
                }
            }
        }
    ))
}
