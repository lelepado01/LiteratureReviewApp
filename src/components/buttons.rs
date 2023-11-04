
use dioxus::prelude::*;

pub fn create_button_open_pdf(cx: Scope, path: String) -> Element {
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
                class: "inline-flex items-center rounded-md bg-indigo-600 px-2 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                svg {
                    class: "h-8 w-8 text-black-500",
                    width: "24",
                    height: "24",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    "stroke-width": "1",
                    stroke: "currentColor",
                    path {
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        d: "M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15m0-3l-3-3m0 0l-3 3m3-3V15"
                    }
                }
            }
        }
    })
}