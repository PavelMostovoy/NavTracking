use crate::pages::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{TrackerPayload, TrackerResponse, TrackerResult};

#[component]
pub(crate) fn Selector(id: i32) -> Element {
    let mut context = use_context::<Signal<TrackerResponse>>();
    rsx! {
        div {
            id: "selector",
            h1 { "Selection page" }
            button {
            onclick: move |_| {
                spawn(async move {
                    context.write().result = send_tracker_request()
                        .await.unwrap_or(TrackerResponse {result: TrackerResult {tracker_name: "".to_string(), data: vec![]}})
                        .result;
                    println!("{:?}", context().result);
                });
            },
            "Get coordinates"
        }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimplifiedData {
    pub lat: i32,
    pub lon: i32,
    pub time: u32,
}


pub async fn send_tracker_request() -> Result<TrackerResponse, reqwest::Error> {
    let client = Client::new();
    let payload = TrackerPayload {
        tracker_id: "70b3d57ed00653c8".to_string(),
        tracker_name: "hellteck Tracker 1".to_string(),
    };

    let res = client
        .post("https://api.mostovoi.org/get_single_track")
        .json(&payload)
        .send()
        .await?;

    let tracker_data = res.json::<TrackerResponse>().await?;
    Ok(tracker_data)
}
