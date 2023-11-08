#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

mod common;
mod app_data;
mod categories;
mod memos;
mod dashboard;
mod global_search; 
mod paper_search;
mod paper_memos;
mod components;
mod data;
mod export;
mod helpers;
mod scholar;

use categories::categories_data::CategoriesData;
use dashboard::dashboard_data::DashboardData;
use memos::memo_data::MemoData;
use app_data::AppPage; 
use common::{create_navbar, create_header};
use categories::render::create_categories_page;
use export::export_data::ExportData;
use global_search::global_search_data::GlobalSearchData;
use memos::render::create_memos_page;
use dashboard::render::create_dashboard_page;
use global_search::render::create_global_search_page;
use paper_search::render::create_paper_search_page;
use paper_memos::render::create_paper_memos_page;
use paper_search::paper_search_data::PaperSearchData;
use export::render::create_export_page;
use export::ExportTo;

fn main() {
    let cfg = Config::default().with_window(
        WindowBuilder::new()
            .with_maximized(true)
            .with_title("Literature Review App"),
    ); 

    dioxus_desktop::launch_cfg(App, cfg);
}

fn App(cx: Scope) -> Element {

    let page: &UseState<AppPage> = use_state(cx, || AppPage::Dashboard);

    let paper_search_data = PaperSearchData::new(cx);
    let dashboard_data = DashboardData::new(cx);
    let categories_data = CategoriesData::new(cx);
    let global_search_data = GlobalSearchData::new(cx);
    let export_data = ExportData::new(cx);
    let memo_data = MemoData::new(cx);

    cx.render(rsx!(
        load_css(cx)
        create_navbar(cx, page)
        create_header(cx, page)
        match page.get() {
            AppPage::Dashboard => create_dashboard_page(cx, dashboard_data),
            AppPage::Categories => create_categories_page(cx, categories_data),
            AppPage::GeneralMemos => create_memos_page(cx, memo_data),
            AppPage::PaperMemos => create_paper_memos_page(cx),
            AppPage::GlobalSearch => create_global_search_page(cx, global_search_data),
            AppPage::PaperSearch => create_paper_search_page(cx, paper_search_data),
            AppPage::ExportBib => create_export_page(cx, ExportTo::Bib, export_data),
            AppPage::ExportText => create_export_page(cx, ExportTo::Text, export_data),
        }
    ))
}

fn load_css(cx: Scope) -> Element {
    cx.render(
        rsx! {
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css"
            }   
        }
    )
}