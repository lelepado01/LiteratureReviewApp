
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};

use crate::components::buttons::create_button_open_pdf;
use crate::common::create_search_bar;
use crate::dashboard::dashboard_data::DashboardData;
use crate::data::updater::update_categories;
use crate::data::loader::{load_papers, load_unique_categories, load_dashboard_table_rows};

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


    let mut data = load_dashboard_table_rows(dashboard_data.search_query.get().to_owned());
    dashboard_data.sorter.sort(data.as_mut_slice());

    let categories = load_unique_categories();

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
                            Th { sorter: dashboard_data.sorter, field: DashboardTableField::AddCategory, "Add Category" }
                            Th { sorter: dashboard_data.sorter, field: DashboardTableField::Open, "PDF"}
                        }
                    }
                    tbody {
                        for table_row in data.iter() {
                            tr {
                                td { "{table_row.file_name}" }
                                td { "{table_row.pages}" }
                                td { "{table_row.author}" }
                                td {
                                    create_category_tags(cx, table_row.categories.clone())
                                }
                                td {
                                    create_button_add_category(cx, "+".to_string(), table_row.file_name.clone(), categories.clone())
                                }
                                td {
                                    create_button_open_pdf(cx, "Open".to_string(), table_row.file_name.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
   })
}

fn create_category_tags(cx: Scope, categories : Vec<String>) -> Element {
    cx.render(rsx! {
        for cat in categories.iter() {
            span {
                class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800",
                "{cat}"
            }
        }
    })
}



pub fn create_button_add_category(cx: Scope, label : String, file_name: String, categories : Vec<String>) -> Element {

    let hidden_box = use_state(cx, || true);
    let category = use_state(cx, || "".to_string());
    
    cx.render(rsx! {
        span {
            class: "sm:ml-3",
            div {
                class: "relative inline-block text-left",
                div {
                    class: "inline-flex items-center overflow-hidden rounded-md border bg-white",
                    button {
                        "type": "button",
                        class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500",
                        onclick: |_| {
                            hidden_box.set(!hidden_box.get());
                        },
                        label.clone(),
                    }
                },
                if *hidden_box.get() {
                    None
                } else {
                    cx.render(rsx!{
                        div {
                            class: "absolute right-0 z-10 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5",
                            for cat in categories.iter() {
                                create_category_option(cx, cat.to_string(), file_name.clone(), category)
                            }
                            hr{class: "border-t border-gray-100"}
                            create_category_adder(cx, file_name.clone(), category)
                        }  
                    })              
                }
            }
        }
    })
}

fn create_category_option<'a>(cx: Scope<'a>, label : String, file_name: String, category_hook : &'a UseState<String>) -> Element<'a> {
    
    let unselected_style = "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900"; 
    let selected_style = "block px-4 py-2 text-sm text-green-600 bg-gray-100";
    
    let papers = load_papers();
    let mut found = false;
    for paper in papers.iter() {
        if paper.file_name == file_name {
            for cat in paper.categories.iter() {
                if cat == &label {
                    found = true;
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
}

fn create_category_adder<'a>(cx: Scope<'a>, file_name: String, category_hook : &'a UseState<String>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "px-4 py-2",
            div{
                class: "flex",
                input {
                    "type": "text",
                    class: "border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md",
                    placeholder: "Add Category",
                    value: category_hook.get().as_str(),
                    oninput: move |e| {
                        category_hook.set(e.value.clone());
                    }
                }
                button {
                    class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500",
                    onclick: move |_| {
                        let category = category_hook.get();
                        update_categories(&file_name, category);

                        category_hook.set("".to_string());
                    },
                    "Add"
                }
            }
        }
    })
}