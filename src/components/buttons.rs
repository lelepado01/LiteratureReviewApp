
use dioxus::prelude::*;

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