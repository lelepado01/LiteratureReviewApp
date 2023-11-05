
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::dashboard::dashboard_table::DashboardTable;
use crate::dashboard::dashboard_data::DashboardData;
use crate::data::updater::update_paper_data;

pub fn create_dashboard_page<'a>(cx: Scope<'a>, dashboard_data : DashboardData<'a>) -> Element<'a> {
    update_paper_data(); 

    cx.render(rsx!(DashboardTable(cx, dashboard_data)))
}