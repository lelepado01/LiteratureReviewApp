use dioxus::prelude::*;

use crate::categories::categories_data::CategoryTag;

pub fn create_category_badge(cx : Scope, category : CategoryTag) -> Element {
    cx.render(rsx!(
        div{
            class: "badge badge-pill rounded-full p-2",
            style : "background-color: {category.color}",
            category.label,
        },
    ))
}