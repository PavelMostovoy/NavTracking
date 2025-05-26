
mod pages;
mod utils;
mod components;
mod config;

use pages::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDate};

pub static TRACKER_OPTIONS: &[(&str, &str)] = &[
    ("70b3d57ed00653c8", "FRAxxxx"),
    ("70b3d57ed00653c7", "FRA2455"),
];

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResponse {
    pub result: TrackerResult,
}

impl Default for TrackerResponse {
    fn default() -> Self {
        TrackerResponse {
            result: TrackerResult {
                tracker_name: "".to_string(),
                data: vec![]
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResult {
    pub tracker_name: String,
    pub data: Vec<utils::SimplifiedData>,
}

#[derive(Debug,Serialize, Clone)]
struct TrackerPayload {
    tracker_id: String,
    tracker_name: String,
    start_time:i64,
    end_time:i64,
}

#[derive(Debug, Clone)]
pub struct SelectedTracker {
    tracker_id: String,
    data: TrackerResponse,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedDate {
    pub date: NaiveDate,
}

const FAVICON: Asset = asset!("static/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("static/assets/main.css");
const CONFIG_TOML: &str = include_str!("../config.toml");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(
        TrackerResponse{ result: TrackerResult{ tracker_name: "".to_string(), data: vec![] }}
    ));
    use_context_provider(|| Signal::new( vec![
        SelectedTracker { 
            tracker_id: "".to_string(),
            data: TrackerResponse::default()
        },
        SelectedTracker {
            tracker_id: "".to_string(),
            data: TrackerResponse::default()
        },
    ]));

    use_context_provider(|| Signal::new(
        SelectedDate{ date: Local::now().date_naive()}));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}




