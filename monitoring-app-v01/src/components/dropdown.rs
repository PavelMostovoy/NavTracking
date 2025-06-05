use crate::utils::{date_to_unix_range, send_tracker_request};
use crate::{SelectedDate, SelectedTracker, TrackerPayload, TrackerResponse, DEFAULT_SELECTOR, TRACKER_OPTIONS};
use dioxus::prelude::*;

#[component]
pub fn DropdownSelector(index: usize) -> Element {
    let mut trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let selected_date = use_context::<Signal<SelectedDate>>();

    let tracker_snapshot = trackers.read()[index].clone();
    let current_id = tracker_snapshot.tracker_id.clone();
    let mut color = "GREEN";

    if index == 0 {
        color = "BLUE";
    }
    if index == 1 {
        color = "RED";
    }
    rsx! {
        div {
            label { "Select {color} sail Number: " }
            select {
                value: "{current_id}",
                onchange: move |event| {
                    let tracker_id = event.value();


                    if tracker_id.is_empty() {
                        trackers.write()[index] = SelectedTracker {
                            tracker_id: "".to_string(),
                            data: TrackerResponse::default(),
                        };
                        return;
                    }

                    let tracker_name = TRACKER_OPTIONS.iter()
                        .find(|x| x.0 == tracker_id)
                        .map(|x| x.1.to_string())
                        .unwrap_or("Unknown".to_string());


                    if let Some((start, end)) = date_to_unix_range(selected_date.read().date) {
                        let payload = TrackerPayload {
                            tracker_id: tracker_id.clone(),
                            tracker_name: tracker_name.clone(),
                            start_time: Option::from(start),
                            end_time: Option::from(end),
                        };

                        spawn(async move {
                            let response = send_tracker_request(payload)
                                .await
                                .unwrap_or_default();
                            trackers.write()[index] = SelectedTracker {
                                    tracker_id: tracker_id.clone(),
                                    data: response,
                                    };

                        });
                    } else {
                        eprintln!("Invalid date provided!");
                    }
                },

                option {
                    value: "Not Selected ...",
                    disabled: false,
                    hidden: false,
                    selected: current_id.is_empty(),
                    "{DEFAULT_SELECTOR}"
                }

                for (id, name) in TRACKER_OPTIONS.iter() {
                    option {
                        value: "{id}",
                        selected: current_id == *id,
                        "{name}",
                    }
                }
            }
        }
    }
}
