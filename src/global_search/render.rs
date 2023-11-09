
use dioxus::prelude::*;

use crate::data::searcher::search_in_all_files_async;
use crate::components::buttons::create_button_open_pdf;
use crate::components::padding::create_padding_block;

use super::{global_search_data::GlobalSearchData, global_search_results::GlobalSearchResult};

pub fn create_global_search_page<'a>(cx: Scope<'a>, global_search_data : GlobalSearchData<'a>) -> Element<'a> {

    cx.render(rsx!(
        create_global_search_bar(cx, global_search_data.clone())
    
        div{
            class : "max-w-7xl mx-auto py-8 px-8 sm:px-6 lg:px-8 items-center",
            for result in global_search_data.search_results.get() {
                create_search_result(cx, result)
            }
        }
    ))
}

fn create_global_search_bar<'a>(cx: Scope<'a>, global_search_data : GlobalSearchData<'a>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "max-w-7xl mx-auto py-4 px-4",
            div {
                class: "relative",
                div{
                    class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                    svg {
                        class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 20 20",
                        "fill": "currentColor",
                        path {
                            "stroke": "currentColor",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
                        }
                    }
                }
                input {
                    class: "w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    "type": "search",
                    id: "default-search",
                    placeholder: "Search in all files",
                    value: global_search_data.search_query.get().as_str(),
                    oninput: move |event| {
                        global_search_data.search_query.set(event.value.clone());
                    },
                }
                button {
                    class: "text-white absolute right-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    onclick: move |_| {
                        let query = global_search_data.search_query.get();
                        let results = search_in_all_files_async(query);
                        
                        pollster::block_on(async {
                            for result in results.await {
                                global_search_data.search_results.with_mut(|results| {
                                    results.push(result);
                                });
                            }
                        });
                    },
                    "Search"
                }
            }
        }
    ))
}

fn create_search_result<'a>(cx : Scope<'a>, result : &'a GlobalSearchResult) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "mx-auto",
            div {
                class: "bg-white shadow overflow-hidden sm:rounded-lg",
                div {
                    class: "px-3 py-5 sm:px-6 flex items-center",
                    h3 {
                        class: "text-lg leading-6 font-medium text-gray-900 overflow-ellipsis overflow-hidden",
                        result.file_name.clone()
                    }
                    create_padding_block(cx)
                    div {
                        class: "flex flex-row w-full justify-end items-end px-2",
                        p {
                            class: "text-sm text-gray-600 w-lg max-w-lg overflow-ellipsis overflow-hidden",
                            result.file_content.clone()
                        }
                        create_padding_block(cx),
                        create_button_open_pdf(cx, result.file_name.clone())
                    }
                }
            }
        }
    ))
}