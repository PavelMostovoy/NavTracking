mod pages;
mod utils;
mod components;
mod config;

use pages::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Local, NaiveDate};

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResponse {
    pub result: TrackerResult,
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
    tracker_name: String,
    tracker_id: String,
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
    use_context_provider(|| Signal::new( 
        SelectedTracker{ tracker_name: "".to_string(), tracker_id: "".to_string() }));

    use_context_provider(|| Signal::new(
        SelectedDate{ date: Local::now().date_naive()}));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}




