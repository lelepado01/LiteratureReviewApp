
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::categories::categories_table::CategoriesTable;
use crate::categories::categories_data::CategoriesData;
use crate::data::updater::add_category_data;
use crate::components::padding::create_padding_block;

pub fn create_categories_page<'a>(cx: Scope<'a>, categories_data : CategoriesData<'a>) -> Element<'a> {
    cx.render(rsx!(
        CategoriesTable(cx, categories_data),
        create_padding_block(cx),
        div{
            class: "flex flex-row items-center justify-center",
            div{
                class: "flex flex-row items-center justify-center",
                input {
                    class: "w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    "type": "text",
                    placeholder: "Category Name",
                    oninput: move |e| {
                        categories_data.category_name_temp.set(e.data.value.clone());
                    },
                }
                button {
                    class: "inline-flex items-center rounded-md bg-indigo-600 p-4 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                    onclick: move |_| {
                        add_category_data(categories_data.category_name_temp.get().clone());
                        categories_data.category_name_temp.set("".to_string());
                    },
                    "New"
                }
            }
        }
    ))
}