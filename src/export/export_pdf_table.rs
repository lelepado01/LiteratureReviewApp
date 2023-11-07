
use dioxus::prelude::*;
use dioxus_sortable::{PartialOrdBy, SortBy, Sortable, Th};
use serde::{Deserialize, Serialize};

use crate::categories::categories_data::CategoryTag;
use crate::data::downloader::{download_paper_citation, DownloaderResult};
use crate::common::create_search_bar;
use crate::data::loader::load_pdf_export_rows;
use crate::components::badges::create_category_badge;

use super::export_data::ExportData;

/// Our table row. Type `T`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportPDFTableRow {
    pub file_name: String,
    pub categories: Vec<CategoryTag>,
}

/// Our table columns. Type `F`. One for each field in Person.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum ExportPDFTableField {
    #[default]
    FileName,
    Category,
    AddRemove,
}

/// Specify how we sort our `Person` using `PersonField`.
impl PartialOrdBy<ExportPDFTableRow> for ExportPDFTableField {
    fn partial_cmp_by(&self, a: &ExportPDFTableRow, b: &ExportPDFTableRow) -> Option<std::cmp::Ordering> {
        // Note how it's just a passthru to `PartialOrd` for each field.
        match self {
            ExportPDFTableField::Category => Some(std::cmp::Ordering::Equal),
            ExportPDFTableField::FileName => Some(std::cmp::Ordering::Equal),
            ExportPDFTableField::AddRemove => Some(std::cmp::Ordering::Equal),
        }
    }
}

/// Specify sorting options available on a column.
impl Sortable for ExportPDFTableField {
    fn sort_by(&self) -> Option<SortBy> {
        // We can choose column specifics but this is good for the minimum.
        SortBy::increasing_or_decreasing()
    }
}

pub fn ExportPDFTable<'a>(cx: Scope<'a>, export_data : ExportData<'a>) -> Element<'a> {

    let mut data = load_pdf_export_rows(); 
    data.retain(|row| row.file_name.to_lowercase().contains(export_data.search_query.get()));
    export_data.sorter.sort(data.as_mut_slice());

    cx.render(rsx!{
        div { 
            class: "mx-auto p-4 bg-gray-100 flex justify-center",
            create_search_bar(cx, export_data.search_query)
            div { 
                class: "p-2"
            }
            div { class: "flex items-center justify-center flex-row",
                table {
                    thead {
                        tr {
                            Th { sorter: export_data.sorter, field: ExportPDFTableField::FileName, "File" }
                            Th { sorter: export_data.sorter, field: ExportPDFTableField::Category, "Category" }
                            Th { sorter: export_data.sorter, field: ExportPDFTableField::AddRemove, "" }
                        }
                    }
                    tbody {
                        for table_row in data.iter() {
                            create_table_row(cx, table_row.clone(), export_data)
                        }
                    }
                }
            }
        }
   })
}

fn create_table_row<'a>(cx : Scope<'a>, table_row : ExportPDFTableRow, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx!(
        tr {
            td { table_row.file_name.clone() }
            td { 
                for category in table_row.categories.iter() {
                    cx.render(rsx!(create_category_badge(cx, category.clone())))
                }
            }
            td { 
                div{
                    class: "flex flex-row items-center justify-center",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            let tr = table_row.file_name.clone();
                            let cit = download_paper_citation(tr);
                            match cit {
                                DownloaderResult::Ok(cit) => export_data.add_citation_data(cit), 
                                _ => {}
                            }
                        },
                        "Add"
                    }
                }
            }
        }
    ))
}