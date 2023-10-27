

use dioxus::prelude::*;
use dioxus_sortable::{use_sorter, PartialOrdBy, SortBy, Sortable, Th};

use crate::common::create_search_bar;
use crate::components::buttons::create_button_open_pdf;


/// Our table row. Type `T`.
#[derive(Clone, Debug, PartialEq)]
pub struct MemoTableRow {
    path: String,
    file_name: String,
    author: String,
    pages: u32,
    categories: Vec<String>,
}

/// Our table columns. Type `F`. One for each field in Person.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum MemoTableField {
    /// Use default for the initial sort.
    #[default]
    FileName,
    Author,
    Pages,
    Categories,
    Open,
}

/// Specify how we sort our `Person` using `PersonField`.
impl PartialOrdBy<MemoTableRow> for MemoTableField {
    fn partial_cmp_by(&self, a: &MemoTableRow, b: &MemoTableRow) -> Option<std::cmp::Ordering> {
        // Note how it's just a passthru to `PartialOrd` for each field.
        match self {
            MemoTableField::FileName => a.file_name.partial_cmp(&b.file_name),
            MemoTableField::Author => a.author.partial_cmp(&b.file_name),
            MemoTableField::Pages => a.pages.partial_cmp(&b.pages),
            MemoTableField::Categories => a.file_name.partial_cmp(&b.file_name),
            MemoTableField::Open => a.file_name.partial_cmp(&b.file_name),
        }
    }
}

/// Specify sorting options available on a column.
impl Sortable for MemoTableField {
    fn sort_by(&self) -> Option<SortBy> {
        // We can choose column specifics but this is good for the minimum.
        SortBy::increasing_or_decreasing()
    }
}

fn get_data(name :&UseState<String>) -> Vec<MemoTableRow> {
    let path = "./papers/"; 
    let files = std::fs::read_dir(path).unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    let mut data = Vec::new();

    for file in files {
        data.push(MemoTableRow {
            path: file.to_str().unwrap().to_string(),
            file_name: file.file_name().unwrap().to_str().unwrap().to_string(),
            author: "author".to_string(),
            pages: 10,
            categories: vec!["category".to_string()],
        });
    }

    data.retain(|x| x.file_name.to_lowercase().contains(name.get()));

    data
}


pub fn MemoTable(cx: Scope) -> Element {

    let sorter = use_sorter::<MemoTableField>(cx);
    let name = use_state(cx, || "".to_string()); 
    let mut data = get_data(name);
    sorter.sort(data.as_mut_slice());

    cx.render(rsx!{
        div { 
            class: "mx-auto p-4 bg-gray-100 flex justify-center",
            create_search_bar(cx, name)
            div { 
                class: "p-2"
            }
            div { class: "flex items-center justify-center flex-row",
                table {
                    thead {
                        tr {
                            Th { sorter: sorter, field: MemoTableField::FileName, "Name" }
                            Th { sorter: sorter, field: MemoTableField::Pages, "Pages" }
                            Th { sorter: sorter, field: MemoTableField::Author, "Author" }
                            Th { sorter: sorter, field: MemoTableField::Categories, "Categories" }
                            Th { sorter: sorter, field: MemoTableField::Open, "PDF"}
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
                                    create_button_open_pdf(cx, "Open".to_string(), table_row.path.clone())
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