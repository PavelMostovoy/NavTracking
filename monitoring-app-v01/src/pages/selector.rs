use crate::pages::Route;
use crate::{SelectedTracker, TrackerPayload, TrackerResponse, TrackerResult};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use reqwest::Client;
use serde::__private::de::Content::String;
use serde::{Deserialize, Serialize};
use std::thread::Scope;

#[component]
pub(crate) fn Selector(id: i32) -> Element {
    let selected_tracker = use_context::<Signal<SelectedTracker>>();
    let context = use_context::<Signal<TrackerResponse>>();

    rsx! {
        div {
            id: "selector",
            h3 { "Tracker and Data selection" }
            div {
                DropdownSelector {}
            }
            button {
                onclick: move |_| {
                },
                "Select Date"
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

pub async fn send_tracker_request(
    tracker_payload: TrackerPayload,
) -> Result<TrackerResponse, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post("https://api.mostovoi.org/get_single_track")
        .json(&tracker_payload)
        .send()
        .await?;

    let tracker_data = res.json::<TrackerResponse>().await?;
    Ok(tracker_data)
}
#[component]
fn DropdownSelector() -> Element {
    let mut selected_tracker = use_context::<Signal<SelectedTracker>>();
    let tracker_data = use_context::<Signal<TrackerResponse>>();

    let options = vec![
        ("70b3d57ed00653c8", "hellteck Tracker 1"),
        ("70b3d57ed00653c7", "hellteck Tracker 2"),
    ];

    rsx! {
        div {
            label { "Select sail Number: " }
            select {
                onchange: move |event| {
                    let tracker_id = event.value();
                    let tracker_name = options.iter()
                    .find(|x| x.0 == tracker_id)
                    .map(|x| x.1.to_string())
                    .unwrap_or("Unknown".to_string());

                    selected_tracker.set(SelectedTracker {
                        tracker_id,
                        tracker_name,
                    });
                   let payload = TrackerPayload {
                            tracker_id: selected_tracker.read().tracker_id.clone(),
                            tracker_name: selected_tracker.read().tracker_name.clone(),
                        };
                    let mut context = tracker_data.clone();

                    spawn(async move {
                        let response = send_tracker_request(payload)
                            .await
                            .unwrap_or(TrackerResponse {
                                result: TrackerResult {
                                    tracker_name: "".to_string(),
                                    data: vec![],
                                }
                            });

                        context.write().result = response.result;
                        println!("Result: {:?}", context.read().result);

                    });
                },
                option {
                    value: "",
                    disabled: true,
                    selected: selected_tracker.read().tracker_id.is_empty(),
                    "Select a tracker..."
                }

                for (id, name) in options.iter() {
                    option {
                        value: "{id}",
                         selected: *id == selected_tracker.read().tracker_id,
                        "{name}",
                    }
                }
            }

            p { "Debug Selected: ID = {selected_tracker.read().tracker_id}, Name = {selected_tracker.read().tracker_name}" }
        }
    }
}
