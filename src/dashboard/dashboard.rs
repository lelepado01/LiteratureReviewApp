
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::tables::dashboard::DashboardTable;
use crate::dashboard::dashboard_data::DashboardData;

pub fn create_dashboard_page<'a>(cx: Scope<'a>, dashboard_data : DashboardData<'a>) -> Element<'a> {
    cx.render(rsx!(DashboardTable(cx, dashboard_data)))
}