
use dioxus::prelude::*;

use crate::export::ExportTo;
use crate::components::padding::create_padding_block;
use crate::export::get_data::{export_to_bib, export_to_text};
use crate::export::export_pdf_table::ExportPDFTable;
use crate::export::export_data::ExportData;

pub fn create_export_page<'a>(cx : Scope<'a>, export : ExportTo, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "items-center justify-center",
            create_pdf_table(cx, export_data)
            div {
                class: "flex flex-col items-center justify-center",
                match export {
                    ExportTo::Bib => create_export_to_bib_list(cx, export_data),
                    ExportTo::Text => create_export_to_text_list(cx, export_data),
                }
                div {
                    class: "flex flex-row items-center justify-center w-full",
                    create_padding_block(cx),
                    create_export_button(cx, export, export_data),
                }
            }
        }
    })
}

fn create_pdf_table<'a>(cx : Scope<'a>, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx!(ExportPDFTable(cx, export_data)))
}

fn create_export_to_bib_list<'a>(cx : Scope<'a>, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center justify-center",
            "TODO"
        }
    })
}

fn create_export_to_text_list<'a>(cx : Scope<'a>, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center justify-center",
            "TODO"
        }
    })
}

fn create_export_button<'a>(cx : Scope<'a>, export : ExportTo, export_data : ExportData<'a>) -> Element<'a> {
    cx.render(rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
            onclick: move |_| {
                match export {
                    ExportTo::Bib => export_to_bib(export_data),
                    ExportTo::Text => export_to_text(export_data),
                }
            },
            "Export to ",
            match export {
                ExportTo::Bib => "Bib",
                ExportTo::Text => "Text",
            }
        }
    })
}