mod components;
mod config;
mod pages;
mod utils;

use crate::utils::Coordinate;
use chrono::{Local, NaiveDate};
use dioxus::desktop::Config;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::Level;
use pages::Route;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const DEFAULT_SELECTOR: &str = "Not Selected ...";

pub static TRACKER_OPTIONS: &[(&str, &str)] = &[
    ("70b3d57ed00653c7", "FRA2455"),
    ("e4cf9ca79006202d", "FRA2456"),
    ("204f19861e8df578", "FRA2457"),
    ("690faa05beeba816", "FRA2459"),
    ("70b3d57ed00653c8", "FRA2460"),
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
                data: vec![],
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackerResult {
    pub tracker_name: String,
    pub data: Vec<utils::SimplifiedData>,
}

#[derive(Debug, Serialize, Clone)]
struct TrackerPayload {
    tracker_id: String,
    tracker_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<i64>,
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

struct MapDisplayState {
    zoom: i32,
    coordinate: Coordinate,
}

const FAVICON: Asset = asset!("static/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("static/assets/main.css");
const CONFIG_TOML: &str = include_str!("../config.toml");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to initialize logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(vec![
            SelectedTracker {
                tracker_id: DEFAULT_SELECTOR.to_string(),
                data: TrackerResponse::default(),
            },
            SelectedTracker {
                tracker_id: DEFAULT_SELECTOR.to_string(),
                data: TrackerResponse::default(),
            },
        ])
    });

    use_context_provider(|| {
        Signal::new(SelectedDate {
            date: Local::now().date_naive(),
        })
    });

    use_context_provider(|| {
        Signal::new(MapDisplayState {
            zoom: 13,
            coordinate: Coordinate::default(),
        })
    });

    spawn(async move {
        tracing::info!("Started new listener");
        let mut zoom_level = use_context::<Signal<MapDisplayState>>();

        let mut eval = document::eval(
            r#"
            window.addEventListener("message", (event) => {
                if (event.data.type === "map_update") {
                    dioxus.send(JSON.stringify(event.data));
                }
            });
        "#,
        );

        while let Ok(message) = eval.recv::<String>().await {
            let parsed: Result<Value, _> = serde_json::from_str(&message);
            let mut map_state = MapDisplayState { zoom: 13, coordinate: Default::default() };
            if let Ok(json)= parsed {
                if let Some(zoom) = json["zoom"].as_str() {
                    tracing::info!("Zoom: {}", zoom);
                    map_state.zoom = zoom.parse::<i32>().unwrap();
                }
                if let Some(lat) = json["center"]["lat"].as_str() {
                    tracing::info!("Lat: {}", lat);
                    map_state.coordinate.lat = lat.parse::<f32>().unwrap();
                }
                if let Some(lng) = json["center"]["lng"].as_str() {
                    tracing::info!("Lng: {}", lng);
                    map_state.coordinate.lon = lng.parse::<f32>().unwrap();
                }
                zoom_level.set(map_state);
            } else {
                tracing::info!("Failed to parse JSON: {}", message);
            }
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
