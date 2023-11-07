
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};

use crate::categories::categories_data::CategoryTag;
use crate::components::buttons::create_button_open_pdf;
use crate::common::create_search_bar;
use crate::components::badges::create_category_badge;
use crate::dashboard::dashboard_data::DashboardData;
use crate::data::updater::update_categories;
use crate::data::loader::{load_papers, load_dashboard_table_rows, load_categories_data, LoaderResult};
use crate::helpers::table_helpers::{handle_table_show_modal_hook, table_show_modal_hook_is_visible};

/// Our table row. Type `T`.
#[derive(Clone, Debug, PartialEq)]
pub struct DashboardTableRow {
    pub file_name: String,
    pub author: String,
    pub pages: u32,
    pub categories: Vec<String>,
}

/// Our table columns. Type `F`. One for each field in Person.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum DashboardTableField {
    /// Use default for the initial sort.
    #[default]
    FileName,
    Author,
    Pages,
    Categories,
    AddCategory,
    Open,
}

/// Specify how we sort our `Person` using `PersonField`.
impl PartialOrdBy<DashboardTableRow> for DashboardTableField {
    fn partial_cmp_by(&self, a: &DashboardTableRow, b: &DashboardTableRow) -> Option<std::cmp::Ordering> {
        // Note how it's just a passthru to `PartialOrd` for each field.
        match self {
            DashboardTableField::FileName => a.file_name.partial_cmp(&b.file_name),
            DashboardTableField::Author => a.author.partial_cmp(&b.file_name),
            DashboardTableField::Pages => a.pages.partial_cmp(&b.pages),
            DashboardTableField::Categories => a.file_name.partial_cmp(&b.file_name),
            DashboardTableField::Open => a.file_name.partial_cmp(&b.file_name),
            DashboardTableField::AddCategory => a.file_name.partial_cmp(&b.file_name),
        }
    }
}

/// Specify sorting options available on a column.
impl Sortable for DashboardTableField {
    fn sort_by(&self) -> Option<SortBy> {
        // We can choose column specifics but this is good for the minimum.
        SortBy::increasing_or_decreasing()
    }
}

pub fn DashboardTable<'a>(cx: Scope<'a>, dashboard_data : DashboardData<'a>) -> Element<'a> {

    let data = load_dashboard_table_rows(dashboard_data.search_query.get().to_owned());
    let categories = load_categories_data();

    match ( data, categories) {
        (LoaderResult::Ok(mut data), LoaderResult::Ok(categories)) => {
            dashboard_data.sorter.sort(data.as_mut_slice());
            cx.render(rsx!{
                div { 
                    class: "mx-auto p-4 bg-gray-100 flex justify-center",
                    create_search_bar(cx, dashboard_data.search_query)
                    div { 
                        class: "p-2"
                    }
                    div { class: "flex items-center justify-center flex-row",
                        table {
                            thead {
                                tr {
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::FileName, "Name" }
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::Pages, "Pages" }
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::Author, "Author" }
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::Categories, "Categories" }
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::AddCategory, "Add" }
                                    Th { sorter: dashboard_data.sorter, field: DashboardTableField::Open, "Open"}
                                }
                            }
                            tbody {
                                for (i, table_row) in data.iter().enumerate() {
        
                                    tr {
                                        td { "{table_row.file_name}" }
                                        td { "{table_row.pages}" }
                                        td { "{table_row.author}" }
                                        td {
                                        {
                                            let mut cats = categories.iter().map(|cat| CategoryTag { label: cat.label.to_string(), color: cat.color.to_string() }).collect::<Vec<CategoryTag>>();
                                            cats.retain(|cat| table_row.categories.contains(&cat.label)); 
                                            create_category_tags(cx, cats)}
                                        }
                                        td {
                                            create_button_add_category(cx, i, table_row.file_name.clone(), categories.clone(), dashboard_data)
                                        }
                                        td {
                                            create_button_open_pdf(cx, table_row.file_name.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
           })
        },
        _ => None
    }
}

fn create_category_tags(cx: Scope, categories : Vec<CategoryTag>) -> Element {
    cx.render(rsx! {
        for cat in categories.iter() {
            create_category_badge(cx, CategoryTag { label: cat.label.to_string(), color: cat.color.to_string() })
        }
    })
}

pub fn create_button_add_category<'a>(cx: Scope<'a>, row_index : usize, file_name: String, categories : Vec<CategoryTag>, mut dashboard_data : DashboardData<'a>) -> Element<'a> {
    
    cx.render(rsx! {
        span {
            class: "sm:ml-3",
            div {
                class: "relative inline-block text-left",
                div {
                    class: "inline-flex items-center overflow-hidden rounded-md border bg-white",
                    button {
                        "type": "button",
                        class: "inline-flex items-center px-2 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-500 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500",
                        onclick: move |_| {
                            dashboard_data.hidden_box_index = handle_table_show_modal_hook(row_index, dashboard_data.hidden_box_index);
                        },
                        svg {
                            class: "w-8 h-8 text-black-500",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1",
                            stroke: "currentColor",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M12 4.5v15m7.5-7.5h-15"
                            }
                        }
                    },
                },
                if table_show_modal_hook_is_visible(row_index, dashboard_data.hidden_box_index) {
                    cx.render(rsx!(
                        cx.render(rsx!(div {
                            class: "absolute right-0 z-10 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5",
                            for cat in categories.iter() {
                                create_category_option(cx, cat.label.to_string(), file_name.clone(), dashboard_data.category)
                            } 
                        }))
                    ))   
                }
            }
        }
    })
}

fn create_category_option<'a>(cx: Scope<'a>, label : String, file_name: String, category_hook : &'a UseState<String>) -> Element<'a> {
    
    let unselected_style = "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"; 
    let selected_style = "block px-4 py-2 text-sm text-green-600 bg-gray-100";
    
    let papers = load_papers();
    if let LoaderResult::Ok(papers) = papers {
        let mut found = false;
        for paper in papers.iter() {
            if paper.file_name == file_name {
                for cat in paper.categories.iter() {
                    if cat == &label {
                        found = true;
                        break;
                    }
                }
            }
        }

        cx.render(rsx! {
            a {
                href: "#",
                class: if found { selected_style } else { unselected_style },
                onclick: move |_| {
                    update_categories(&file_name, &label);
                    category_hook.set("".to_string());
                },
                label.clone()
            }
        })
    } else {
        None
    }
}