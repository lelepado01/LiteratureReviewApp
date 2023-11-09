
use dioxus::prelude::*;

use crate::data::updater::update_memo_data;

use super::general_memo_data::{MemoData, Memo};

pub fn create_memos_page<'a>(cx: Scope<'a>, memo_data : MemoData<'a>) -> Element<'a> {

    let memos: Vec<Memo> = memo_data.all_memos.read().to_owned();

    cx.render(rsx!(
        div {
            class: "w-full p-4 bg-gray-100",
            create_add_memo_form(cx, memo_data),
            ul {
                class: "m-8 mx-24 relative space-y-2",
                for memo in memos.iter() {
                    create_memo_list_item(cx, memo.clone(), memo_data)
                }
            }
            if !memo_data.add_memo_modal_form_hidden.get() {
                create_add_memo_modal_form(cx, memo_data)
            }
        }
    ))
}

fn create_add_memo_form<'a>(cx : Scope<'a>, memo_data : MemoData<'a>) -> Element<'a> {
    cx.render(rsx!(
        div{
            class: "rounded-lg m-8 mx-24 relative space-y-2 bg-white rounded-lg",
            div {
                class: "flex flex-row justify-between items-center",
                input {
                    class: "w-full h-12 px-4 bg-white-100 rounded-lg focus:outline-none",
                    "type": "text",
                    placeholder: "Enter memo here",
                    oninput: move |e| {
                        memo_data.modal_form_memo.set(Memo { content: e.value.clone(), open:false, done:false, children: vec![] });
                    }
                }
                button {
                    class: "mx-1 p-2 h-12 rounded-lg bg-indigo-600 hover:bg-indigo-500 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500",
                    onclick: move |_| {
                        memo_data.add_memo(memo_data.modal_form_memo.get().clone());
                        update_memo_data(memo_data.all_memos.read().to_owned());
                        memo_data.modal_form_memo.set(Memo { content: "".to_string(), open:false, done:false, children: vec![] });
                    },
                    svg {
                        class: "w-8 h-8 text-white-500",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        "stroke-width": "1",
                        stroke: "white",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 4.5v15m7.5-7.5h-15"
                        }
                    }
                }
            }
        }
    ))
}

fn create_memo_list_item<'a>(cx : Scope<'a>, memo : Memo, memo_data : MemoData<'a>) -> Element<'a> {
    let checked_style = "mx-1 p-2 h-12 rounded-full bg-green-600 hover:bg-green-500 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-green-500"; 
    let unchecked_style = "mx-1 p-2 h-12 rounded-full bg-gray-600 hover:bg-gray-500 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-500";

    let open_svg = rsx!(
        svg {
            class: "w-8 h-8 text-white-500",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": "1",
            stroke: "black",
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M19 9l-7 7-7-7"
            }            
        }
    );

    let closed_svg = rsx!(
        svg {
            class: "w-8 h-8 text-white-500",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            "viewBox": "0 0 24 24",
            "stroke-width": "1",
            stroke: "black",
            path {
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                d: "M5 15l7-7 7 7"
            }
        }
    );

    let content1 = memo.content.clone();
    let content2 = memo.content.clone();
    let content3 = memo.content.clone();

    let children = memo.children.clone();

    cx.render(rsx!(
        div{
            class: "p-5 bg-white rounded-lg flex items-end justify-right",
            div{
                class: "flex-1 flex",
                div{
                    onclick: move |_| {
                        memo_data.toggle_memo(content3.clone());
                        update_memo_data(memo_data.all_memos.read().to_owned());
                    },
                    if memo.open { cx.render(open_svg) } else { cx.render(closed_svg) }   
                }
                p {
                    class:  "text-2xl font-bold text-gray-800",
                    memo.content.clone()
                }
                div{
                    class: "flex flex-row w-full justify-end items-end px-2",
                    cx.render(rsx!(
                        div {
                            class: if memo.done { checked_style } else { unchecked_style },
                            onclick: move |_| {
                                // check memo
                                memo_data.check_memo(memo.content.clone());
                                update_memo_data(memo_data.all_memos.read().to_owned());
                            },
                            svg {
                                class: "w-8 h-8 text-white-500",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1",
                                stroke: "white",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M5 13l4 4L19 7"
                                }
                            }
                        }
                    ))
                    div {
                        class: "mx-1 p-2 h-12 rounded-lg  bg-indigo-600 hover:bg-indigo-500 focus-visible:outline focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-indigo-500",
                        onclick: move |_| {
                            // add child
                            memo_data.memo_parent.set(content1.clone());
                            memo_data.add_memo_modal_form_hidden.set(false);
                            memo_data.modal_form_memo.set(Memo { content: "".to_string(), open:false, done:false, children: vec![] });
                        },
                        svg {
                            class: "w-8 h-8 text-white-500",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1",
                            stroke: "white",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M12 4.5v15m7.5-7.5h-15"
                            }
                        }
                    }
                    div {
                        class: "mx-1 p-2 h-12 rounded-lg bg-red-800 hover:bg-red-600",
                        onclick: move |_| {
                            // remove memo
                            memo_data.remove_memo(content2.clone());
                            update_memo_data(memo_data.all_memos.read().to_owned());    
                        },
                        svg{
                            class: "h-8 w-8 text-white-500",
                            width: "24",
                            height: "24",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1",
                            stroke: "white",
                            fill: "none",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            polyline {
                                points: "3 6 5 6 21 6"
                            }
                            path {
                                d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                            }
                            line {
                                x1: "10",
                                y1: "11",
                                x2: "10",
                                y2: "17"
                            }
                            line {
                                x1: "14",
                                y1: "11",
                                x2: "14",
                                y2: "17"
                            }
                        }
                    }
                }
            }
        },
        if memo.open {
            cx.render(rsx!(
                ul {
                    class: "my-8 ml-24 relative space-y-2",
                    for child in children.iter() {
                        create_memo_list_item(cx, child.clone(), memo_data)
                    }
                }
            ))
        } else {
            None
        }
        
    ))

}

fn create_add_memo_modal_form<'a>(cx : Scope<'a>, memo_data : MemoData<'a>) -> Element<'a> {

    cx.render(rsx!(
        div {
            class: "fixed z-10 inset-0 overflow-y-auto",
            div {
                class: "transition-opacity flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
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
                    class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div {
                        class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        div {
                            class: "mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left",
                            h3 {
                                class: "text-lg leading-6 font-medium text-gray-900",
                                id: "modal-title",
                                "Add Memo"
                            }
                            div {
                                class: "mt-2",
                                input {
                                    class: "w-full h-12 px-4 bg-white-100 rounded-lg focus:outline-none",
                                    "type": "text",
                                    placeholder: "Enter memo here",
                                    oninput: move |e| {
                                        memo_data.modal_form_memo.set(Memo { content: e.value.clone(), open:false, done:false, children: vec![] });
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse",
                        div {
                            class: "mt-3 sm:mt-0 sm:ml-3 sm:flex-shrink-0",
                            button {
                                "type": "button",
                                class: "w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:ml-3 sm:w-auto sm:text-sm",
                                onclick: move |_| {
                                    memo_data.add_memo_to_memo(memo_data.memo_parent.get().clone(), memo_data.modal_form_memo.get().clone());
                                    update_memo_data(memo_data.all_memos.read().to_owned());
                                    memo_data.add_memo_modal_form_hidden.set(true);
                                },
                                "Save"
                            }
                            button {
                                "type": "button",
                                class: "mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:w-auto sm:text-sm",
                                onclick: move |_| {
                                    memo_data.add_memo_modal_form_hidden.set(true);
                                    memo_data.modal_form_memo.set(Memo { content: "".to_string(), open:false, done:false, children: vec![] });
                                },
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    ))
}