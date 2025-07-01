use crate::utils::{date_to_unix_range, send_tracker_request};
use crate::{SelectedDate, SelectedTracker, TrackerPayload, TrackerResponse, DEFAULT_SELECTOR, TRACKER_OPTIONS};
use dioxus::prelude::*;
use dioxus_logger::tracing;

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
            class: "dropdown-container",
            label { 
                class: "dropdown-label",
                "Select {color} Tracker" 
            }
            div {
                class: "custom-select-wrapper",
                select {
                    class: "custom-select",
                    value: "{current_id}",
                    onchange: move |event| {
                        let tracker_id = event.value();

                        // Show loading state (in a real app, you'd update a loading state)
                        tracing::info!("Loading tracker data for {}", tracker_id);

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

                                // Success notification (in a real app, you'd update a success state)
                                tracing::info!("Successfully loaded tracker data for {}", tracker_id);
                            });
                        } else {
                           tracing::warn!("Invalid date provided!");
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
                div { class: "select-arrow" }
            }

            div {
                class: "dropdown-helper-text",
                if current_id.is_empty() {
                    "Please select a tracker to view its data"
                } else {
                    "Tracker selected: {TRACKER_OPTIONS.iter().find(|x| x.0 == current_id).map(|x| x.1).unwrap_or(\"Unknown\")}"
                }
            }
        }
    }
}
