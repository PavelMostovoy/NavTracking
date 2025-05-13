use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::{SelectedTracker, SelectedDate, TrackerPayload, TrackerResponse, TrackerResult};
use crate::utils::{date_to_unix_range, send_tracker_request};


#[component]
pub fn DropdownSelector() -> Element {
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
                     selected: selected_tracker.read().tracker_id.is_empty(),
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
