use dioxus::prelude::*;

use crate::categories::categories_data::CategoryTag;

pub fn create_category_badge(cx : Scope, category : CategoryTag) -> Element {
    cx.render(rsx!(
        div{
            class: "badge badge-pill p-1 m-1 rounded-full",
            style : "background-color: {category.color}; text-align: center;",
            category.label,
        },
    ))
}