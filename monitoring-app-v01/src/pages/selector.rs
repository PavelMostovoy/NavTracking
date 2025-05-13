use crate::pages::Route;
use crate::{SelectedDate, SelectedTracker, TrackerPayload, TrackerResponse, TrackerResult};
use chrono::{NaiveDate, TimeZone, Utc};
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::thread::Scope;

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
pub(crate) fn Selector(id: i32) -> Element {
    rsx! {
        div {
            id: "selector",
            h3 { "Tracker and Data selection" }
            div {
                DropdownSelector {}
            }
            div{
                DateSelector {}
            }
            button {
                onclick: move |_| {
                },
                "Online Monitoring"
            }
        }
    }
}
#[component]
fn DropdownSelector() -> Element {
    let mut selected_tracker = use_context::<Signal<SelectedTracker>>();
    let tracker_data = use_context::<Signal<TrackerResponse>>();
    let selected_date = use_context::<Signal<SelectedDate>>();

    let options = vec![
        ("70b3d57ed00653c8", "hellteck Tracker 1"),
        ("70b3d57ed00653c7", "hellteck Tracker 2"),
    ];

    let current_id = selected_tracker.read().tracker_id.clone();

    rsx! {
        div {
            label { "Select sail Number: " }
            select {
                value: "{current_id}",
                onchange: move |event| {
                    let tracker_id = event.value();
                    let tracker_name = options.iter()
                        .find(|x| x.0 == tracker_id)
                        .map(|x| x.1.to_string())
                        .unwrap_or("Unknown".to_string());

                    selected_tracker.set(SelectedTracker {
                        tracker_id: tracker_id.clone(),
                        tracker_name,
                    });

                    if let Some((start, end)) = date_to_unix_range(selected_date.read().date) {
                        let payload = TrackerPayload {
                            tracker_id,
                            tracker_name: selected_tracker.read().tracker_name.clone(),
                            start_time: start,
                            end_time: end,
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
                    } else {
                        eprintln!("Invalid date provided!");
                    }
                },
                option {
                    value: "",
                    disabled: true,
                    hidden: true,
                    "Select a tracker..."
                }

                for (id, name) in options.iter() {
                    option {
                        value: "{id}",
                        "{name}",
                    }
                }
            }

            p {
                "Debug Selected: ID = {selected_tracker.read().tracker_id}, \
                 Name = {selected_tracker.read().tracker_name}"
            }
        }
    }
}
#[component]
pub fn DateSelector() -> Element {
    let mut selected_date = use_context::<Signal<SelectedDate>>();

    rsx! {
        div {
            label { "Select a date: " }
            input {
                r#type: "date",
                value: "{selected_date.read().date}",
                onchange: move |evt| {
                    if let Ok(date) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
                    selected_date.set(SelectedDate { date });
                }
                }
            }
        }
    }
}

fn date_to_unix_range(date: NaiveDate) -> Option<(i64, i64)> {
    let start_of_day = date.and_hms_opt(0, 0, 0)?;
    let end_of_day = date.and_hms_opt(23, 59, 59)?;

    let start_timestamp = Utc.from_utc_datetime(&start_of_day).timestamp();
    let end_timestamp = Utc.from_utc_datetime(&end_of_day).timestamp();

    Some((start_timestamp, end_timestamp))
}
