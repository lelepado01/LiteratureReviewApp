
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::app_data::AppPage;
use crate::components::padding::create_padding_block;

pub fn create_header<'a>(cx : Scope<'a>, page: &'a UseState<AppPage>) -> Element<'a> {

    cx.render(rsx! {
        header {
            class: "bg-white shadow",
            div {
                class: "max-w-7xl mx-auto py-8 px-8 sm:px-6 lg:px-8 flex items-center",
                img{
                    class: "h-8 w-auto",
                    src: match page.get() {
                        AppPage::Dashboard => "assets/dashboard_icon.png",
                        AppPage::Categories => "assets/categories_icon.png",
                        AppPage::Memos => "assets/memos_icon.png",
                        AppPage::GlobalSearch => "assets/global_search_icon.png",
                        AppPage::PaperSearch => "assets/paper_search_icon.png",
                        AppPage::ExportBib => "assets/export_icon.png",
                        AppPage::ExportText => "assets/export_icon.png",
                    },
                    alt: "Literature Review App"
                }
                create_padding_block(cx)
                h1 {
                    class: "text-3xl font-bold text-gray-900",
                    match page.get() {
                        AppPage::Dashboard => "Dashboard",
                        AppPage::Categories => "Categories",
                        AppPage::Memos => "Memos",
                        AppPage::GlobalSearch => "Search in all Files",
                        AppPage::PaperSearch => "Search for Paper",
                        AppPage::ExportBib => "Export as BibTex File",
                        AppPage::ExportText => "Export as Text File",
                    }
                }

            }
        }
    })
}

pub fn create_navbar<'a>(cx : Scope<'a>, page : &'a UseState<AppPage>) -> Element<'a> {
    let current_page_icon_class = "text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium bg-gray-900";
    let other_page_icon_class = "text-white rounded-md px-3 py-2 text-sm font-medium hover:bg-gray-700 hover:text-white";
    
    let export_dropdown_hidden = use_state(cx, || true);

    cx.render(rsx! {
        nav {
            class: "bg-gray-800",
            div {
                class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex h-16 items-center justify-between",
                    div {
                        class: "flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex flex-shrink-0 items-center",
                            img {
                                class: "h-8 w-auto",
                                src: "assets/literature_icon.png",
                                alt: "Literature Review App"
                            }
                        },
                        div {
                            class: "hidden sm:ml-6 sm:block",
                            div {
                                class: "flex space-x-4",
                                a {
                                    href: "#",
                                    onclick: |_| { 
                                        export_dropdown_hidden.set(true);
                                        page.set(AppPage::Dashboard); 
                                    },
                                    class: if page.out() == AppPage::Dashboard { current_page_icon_class } else { other_page_icon_class },
                                    "Dashboard"
                                },
                                a {
                                    href: "#",
                                    onclick:  |_| { 
                                        export_dropdown_hidden.set(true);
                                        page.set(AppPage::Categories); 
                                    },
                                    class: if page.out()  == AppPage::Categories { current_page_icon_class } else { other_page_icon_class },
                                    "Categories"
                                },
                                a {
                                    href: "#",
                                    onclick: |_| { 
                                        export_dropdown_hidden.set(true);
                                        page.set(AppPage::Memos); 
                                    },
                                    class: if page.out() == AppPage::Memos { current_page_icon_class } else { other_page_icon_class },
                                    "Memos"
                                },
                                a {
                                    href: "#",
                                    onclick: |_| { 
                                        export_dropdown_hidden.set(true);
                                        page.set(AppPage::GlobalSearch); 
                                    },
                                    class: if page.out() == AppPage::GlobalSearch { current_page_icon_class } else { other_page_icon_class },
                                    "Search"
                                },
                                a {
                                    href: "#",
                                    onclick: |_| { 
                                        export_dropdown_hidden.set(true);
                                        page.set(AppPage::PaperSearch); 
                                    },
                                    class: if page.out() == AppPage::PaperSearch { current_page_icon_class } else { other_page_icon_class },
                                    "Papers"
                                },
                                a {
                                    href: "#",
                                    onclick: |_| { export_dropdown_hidden.set(!*export_dropdown_hidden.get()); },
                                    class: if page.out() == AppPage::ExportBib { current_page_icon_class } else { other_page_icon_class },
                                    "Export",
                                    if !export_dropdown_hidden.get() {
                                        cx.render(rsx!(
                                            div {
                                                class: "absolute mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5",
                                                div {
                                                    class: "py-1",
                                                    a {
                                                        href: "#",
                                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        onclick: |_| { 
                                                            page.set(AppPage::ExportBib); 
                                                            export_dropdown_hidden.set(true);
                                                        },
                                                        "Export as BibTex", 
                                                    },
                                                    a {
                                                        href: "#",
                                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        onclick: |_| { 
                                                            page.set(AppPage::ExportText); 
                                                            export_dropdown_hidden.set(true);
                                                        },
                                                        "Export as Text",
                                                    },
                                                },
                                            }
                                        ))
                                    }
                                },
                            }
                        }
                    },
                }
            }
        }
    })
}

pub fn create_search_bar<'a>(cx: Scope<'a>, name :&'a UseState<String>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "max-w-4xl py-4 px-4 items-center",
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
                    value: name.get().as_str(),
                    oninput: move |evt| name.set(evt.value.clone().to_lowercase()),
                }            
            }
        }
    })
}
