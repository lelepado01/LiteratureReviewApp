
use dioxus::prelude::*;

use crate::tables::categories::CategoriesTableRow;
use crate::data::loader::load_categories;

pub fn create_button_open_pdf(cx: Scope, label : String, path: String) -> Element {
    cx.render(rsx! {
        span {
            class: "sm:ml-3",
            button {
                "type": "button",
                onclick: move |_| {
                    let path = path.clone();
                    let _ = std::process::Command::new("open")
                        .arg("papers/".to_owned() + &path)
                        .spawn();
                },
                class: "inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                label
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
    
    let mut categories = load_categories();

    let mut found = false;
    for row in categories.iter_mut() {
        if row.category == label {
            for path in row.paths.iter_mut() {
                if path == &file_name {
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
                let mut categories = load_categories();

                if !found {
                    for row in categories.iter_mut() {
                        if row.category == label.clone() {
                            row.paths.push(file_name.clone());
                        }
                    }
                } else {
                    for row in categories.iter_mut() {
                        if row.category == label {
                            row.paths.retain(|path| path != &file_name);
                        }
                    }
                }

                let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
                ron::ser::to_writer(&mut file, &categories).unwrap();

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
                        let category = category_hook.get().clone();
                        let file_name = file_name.clone();
                        let mut categories = load_categories();

                        let mut found = false;
                        for row in categories.iter_mut() {
                            if row.category == category {
                                found = true;
                            }
                        }

                        if !found {
                            categories.push(CategoriesTableRow {
                                category: category.clone(),
                                paths: vec![file_name.clone()], 
                            });
                        } 

                        let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
                        ron::ser::to_writer(&mut file, &categories).unwrap();

                        category_hook.set("".to_string());
                    },
                    "Add"
                }
            }
        }
    })
}