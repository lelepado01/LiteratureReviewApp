
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};

use crate::components::buttons::{create_button_open_pdf, create_button_add_category};
use crate::common::create_search_bar;
use crate::dashboard::dashboard_data::DashboardData;
use crate::data::loader::{load_categories, load_unique_categories};
use crate::data::updater::update_categories;

/// Our table row. Type `T`.
#[derive(Clone, Debug, PartialEq)]
pub struct DashboardTableRow {
    file_name: String,
    author: String,
    pages: u32,
    categories: Vec<String>,
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

fn get_data(name :String) -> Vec<DashboardTableRow> {

    update_categories();
    
    let data = load_categories();

    let papers = std::fs::read_dir("./papers/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    let mut result = Vec::new();

    for paper in papers.iter() {
        let file_name = paper.file_name().unwrap().to_str().unwrap().to_string();
        let author = "author".to_string(); // TODO: get author from pdf
        let pages = 1; // TODO: get pages from pdf
        let mut categories = Vec::new();

        for row in data.iter() {
            for path in row.paths.iter() {
                if path == &file_name && !categories.contains(&row.category) {
                    categories.push(row.category.clone());
                }
            }
        }

        if name.is_empty() 
            || file_name.to_lowercase().contains(&name) 
            || author.to_lowercase().contains(&name)
            || categories.iter().any(|cat| cat.to_lowercase().contains(&name))
            {
            result.push(DashboardTableRow {
                file_name,
                author,
                pages,
                categories,
            });
        }
    }

    result
}


pub fn DashboardTable<'a>(cx: Scope<'a>, dashboard_data : DashboardData<'a>) -> Element<'a> {

    let mut data = get_data(dashboard_data.search_query.get().to_owned());
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