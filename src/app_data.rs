use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum AppPage {
    Dashboard, 
    Categories,
    Memos,
    GlobalSearch,
    PaperSearch, 
}