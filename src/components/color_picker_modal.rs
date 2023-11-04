
use dioxus::prelude::*;

use crate::data::updater::update_category_color;
use crate::components::badges::create_category_badge;
use crate::categories::categories_data::CategoryTag;

pub fn create_color_picker_modal<'a>(
    cx : Scope<'a>, 
    category : String,
    modal_state_hidden : &'a UseState<std::option::Option<usize>>,
    modal_state_color : &'a UseState<String>
) -> Element<'a> {

    cx.render(rsx!(
        div {
            class: "fixed z-10 inset-0 overflow-y-auto",
            div {
                class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div {
                    class: "inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div {
                        class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4",
                        div {
                            class: "sm:flex sm:items-start",
                            div {
                                class: "mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left",
                                h3 {
                                    class: "text-lg leading-6 font-medium text-gray-900",
                                    id: "modal-title",
                                    "Pick Color"
                                }
                                div {
                                    class: "mt-2",
                                    div {
                                        class: "flex flex-row",
                                        div {
                                            class: "flex flex-col",
                                            div {
                                                class: "flex flex-row justify-between items-center",
                                                img {
                                                    src: "assets/colormap.jpg",
                                                    draggable: false,
                                                    onclick: |e : MouseEvent| {
                                                        e.stop_propagation();
                                                        let target = e.data.element_coordinates(); 
                                                        let color = get_color_from_image(target.x, target.y);
                                                        modal_state_color.set(color);
                                                    },
                                                    ondrag: |e : DragEvent| {
                                                        let target = e.data.mouse.element_coordinates();
                                                        let color = get_color_from_image(target.x, target.y);
                                                        modal_state_color.set(color);
                                                    },
                                                }
                                            },
                                            div {
                                                class: "flex flex-row justify-between items-center",
                                                p {
                                                    class: "text-sm text-gray-500",
                                                    "Selected Color: "
                                                }
                                                div {
                                                    create_category_badge(cx, CategoryTag { label: category.clone(), color: modal_state_color.get().clone() })
                                                }
                                            }
                                        }
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
                                    update_category_color(category.clone(), modal_state_color.get().clone());
                                    modal_state_hidden.set(None);
                                },
                                "Save"
                            }
                            button {
                                "type": "button",
                                class: "mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:w-auto sm:text-sm",
                                onclick: |_| {
                                    modal_state_hidden.set(None);
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


fn get_color_from_image(target_x : f64, target_y : f64) -> String {
    let imagesize = 730;

    let x = target_x as f32 / imagesize as f32;
    let y = target_y as f32 / imagesize as f32;

    // convert coords on color wheel to rgb values
    let r = (1.0 - x) * 255.0;
    let g = (1.0 - y) * 255.0;
    let b = (1.0 - x * y) * 255.0;

    println!("r: {}, g: {}, b: {}", r, g, b);
    format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8)
}