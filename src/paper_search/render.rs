

use dioxus::prelude::{*, SvgAttributes};

use crate::components::padding::create_padding_block;
use crate::paper_search::paper_search_data::PaperSearchData;
use crate::paper_search::paper_search_results::{PaperSearchResult, search_paper_online};
use crate::paper_search::get_data::search_abstract;
use crate::data::updater::add_paper_data;
use crate::data::downloader::download_paper;

pub fn create_paper_search_page<'a>(cx: Scope<'a>, paper_search_data : PaperSearchData<'a>) -> Element<'a> {

    cx.render(rsx!(
        create_paper_search_bar(cx, paper_search_data.search_query, paper_search_data.search_results)
        create_paper_search_list(cx, paper_search_data.clone())
        if !paper_search_data.abstract_modal_hidden.get() {
            create_paper_abstract_modal(cx, paper_search_data.abstract_modal_data, paper_search_data.abstract_modal_hidden)
        }
    ))
}

fn create_paper_search_bar<'a>(cx: Scope<'a>, search_query: &'a UseState<String>, search_results: &'a UseState<Vec<PaperSearchResult>>) -> Element<'a> {
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
                    value: search_query.get().as_str(),
                    oninput: move |event| {
                        search_query.set(event.value.clone());
                    },
                }
                button {
                    class: "text-white absolute right-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                    onclick: move |_| {
                        let query = search_query.get();
                        let results = search_paper_online(query);
                        search_results.set(results);
                    },
                    "Search"
                }
            }
        }
    ))
}

fn create_paper_search_list<'a>(cx: Scope<'a>, paper_search_data : PaperSearchData<'a>) -> Element<'a> {
    cx.render(rsx!(
        div{
            class : "max-w-7xl mx-auto py-8 px-8 sm:px-6 lg:px-8 items-center",
            for result in paper_search_data.search_results.get() {
                create_paper_search_result(cx, result, paper_search_data.abstract_modal_data, paper_search_data.abstract_modal_hidden)
            }
        }
    ))
}

fn create_paper_search_result<'a>(cx : Scope<'a>, search_result: &'a PaperSearchResult, abstract_data : &'a UseState<String>, abstract_modal_hidden : &'a UseState<bool>) -> Element<'a> {
    
    cx.render(rsx!(
        div {
            class: "m-2 bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg flex items-center",
            div {
                class: "px-4 py-5 sm:p-6 flex flex-col justify-between w-2xl max-w-2xl",
                h3 {
                    class: "text-lg leading-6 font-medium text-gray-900 dark:text-white overflow-hidden overflow-ellipsis whitespace-nowrap",
                    search_result.file_name.clone()
                }
                p {
                    class: "mt-1 max-w-1l text-sm text-gray-500 dark:text-gray-400 overflow-hidden overflow-ellipsis whitespace-nowrap",
                    search_result.file_content.clone()
                }
            }
            create_padding_block(cx)
            div {
                class: "px-4 py-5 sm:p-6",
                dl {
                    class: "grid grid-cols-1 gap-16 sm:grid-cols-2 sm:gap-y-12",
                    div {
                        class: "sm:col-span-1",
                        dt {
                            class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                            "Author"
                        }
                        dd {
                            class: "mt-1 text-sm text-gray-900 dark:text-white",
                            search_result.author.replace('\n', ". ").as_str()
                        }
                    }
                    div {
                        class: "sm:col-span-1",
                        dt {
                            class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                            "Year"
                        }
                        dd {
                            class: "mt-1 text-sm text-gray-900 dark:text-white",
                            search_result.year.as_str()
                        }
                    }
                }
            }
            create_padding_block(cx),
            div{
                class: "flex flex-1 flex-row justify-end items-end",
                if search_result.download_link.ends_with(".pdf") {
                    cx.render(rsx!(
                        div {
                            class: "sm:col-span-2",
                            div{
                                a {
                                    class: "w-full flex items-center justify-center p-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                                    "href": "#",
                                    onclick: move |_| {
                                        let paper_data = download_paper(search_result.download_link.clone(), search_result.file_name.clone(), search_result.author.clone());
                                        if paper_data.is_ok() {
                                            add_paper_data(paper_data.unwrap());
                                        }
                                    },
                                    svg {
                                        class: "w-8 h-8",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        "fill": "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1",
                                        "stroke": "white",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            d: "M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0l-3-3m3 3l3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z"
                                        }
                                    },
                                }
                            }
                        }
                    ))
                } else {
                    cx.render(rsx!(div { class: "p-8" }))
                }
                create_padding_block(cx)
                div {
                    class: "sm:col-span-2",
                    div{
                        a {
                            class: "w-full flex items-center justify-center p-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                            "href": "#",
                            onclick: move |_| {
                                abstract_data.set(search_abstract(search_result.page_link.clone()));
                                abstract_modal_hidden.set(false);
                            },
                            svg{
                                class: "w-8 h-8",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1",
                                "stroke": "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M12 7.5h1.5m-1.5 3h1.5m-7.5 3h7.5m-7.5 3h7.5m3-9h3.375c.621 0 1.125.504 1.125 1.125V18a2.25 2.25 0 01-2.25 2.25M16.5 7.5V18a2.25 2.25 0 002.25 2.25M16.5 7.5V4.875c0-.621-.504-1.125-1.125-1.125H4.125C3.504 3.75 3 4.254 3 4.875V18a2.25 2.25 0 002.25 2.25h13.5M6 7.5h3v3H6v-3z"
                                }
                            }
                        }
                    }
                }
                create_padding_block(cx)
                div {
                    class: "sm:col-span-2",
                    div{
                        a {
                            class: "w-full flex items-center justify-center p-3 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                            "href": search_result.page_link.as_str(),
                            "target": "_blank",
                            svg{
                                class: "w-8 h-8",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1",
                                "stroke": "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"
                                }
                            }
                        }
                    }
                }
                create_padding_block(cx),
            }
        }
    ))
}

fn create_paper_abstract_modal<'a>(cx : Scope<'a>, abstract_data : &'a UseState<String>, abstract_modal_hidden : &'a UseState<bool>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "fixed z-10 inset-0 overflow-y-auto",
            div {
                class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div {
                    class: "fixed inset-0 transition-opacity",
                    aria_hidden: "true",
                    div {
                        class: "absolute inset-0 bg-gray-500 opacity-75"
                    }
                }
                span {
                    class: "hidden sm:inline-block sm:align-middle sm:h-screen",
                    aria_hidden: "true",
                }
                div {
                    class: "inline-block align-bottom bg-white dark:bg-gray-800 rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    role: "dialog",
                    aria_modal: "true",
                    aria_labelledby: "modal-headline",
                    div {
                        class: "bg-white dark:bg-gray-800 px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        div {
                            class: "sm:flex sm:items-start",
                            div {
                                class: "mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left",
                                h3 {
                                    class: "text-lg leading-6 font-medium text-gray-900 dark:text-white",
                                    id: "modal-headline",
                                    "Abstract"
                                }
                                div {
                                    class: "mt-2",
                                    p {
                                        class: "text-sm text-gray-500 dark:text-gray-400",
                                        abstract_data.get().as_str()
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "bg-gray-50 dark:bg-gray-700 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse",
                        button {
                            class: "w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-700 text-base font-medium text-white hover:bg-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm",
                            onclick: move |_| { abstract_modal_hidden.set(true); },
                            "Close"
                        }
                    }
                }
            }
        }
    ))
}