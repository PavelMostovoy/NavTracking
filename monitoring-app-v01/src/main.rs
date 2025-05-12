mod pages;
mod utils;

use pages::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResponse {
    pub result: TrackerResult,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResult {
    pub tracker_name: String,
    pub data: Vec<crate::pages::selector::SimplifiedData>,
}

#[derive(Debug,Serialize, Clone)]
struct TrackerPayload {
    tracker_id: String,
    tracker_name: String,
}

#[derive(Debug, Clone)]
pub struct SelectedTracker {
    tracker_name: String,
    tracker_id: String,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}




