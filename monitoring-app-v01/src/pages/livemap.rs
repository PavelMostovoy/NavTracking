use crate::utils::{
    average_geographic_position, generate_markers, generate_polyline, send_tracker_request_actual, Coordinate,
};
use crate::{MapDisplayState, SelectedTracker, SliderValue, TrackerPayload, DEFAULT_SELECTOR, TRACKER_OPTIONS};
use dioxus::prelude::*;

#[component]
pub(crate) fn LiveMap() -> Element {
    let trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let mut slider_value = use_signal(|| 1);
    let map_state = use_context::<Signal<MapDisplayState>>();

    let mut html = include_str!("../../static/assets/map_template.html").to_string();
    html = html.replace("<!--ZOOM_LEVEL-->", map_state.read().zoom.to_string().as_str());
    html = html.replace("<!--START_LAT-->", map_state.read().coordinate.lat.to_string().as_str());
    html = html.replace("<!--START_LON-->", map_state.read().coordinate.lon.to_string().as_str());

    // Trigger data load when component mounts and when slider value changes
    use_effect(move || {
        let current_value = slider_value.read().clone();
        spawn(async move {
            update_tracker_data(current_value, trackers).await;
        });
    });

    // Generate HTML with the current tracker data
    let html = use_memo(move || {
        html = add_tracker_trace(html.clone(), trackers);
        html.clone()
    });


    rsx! {
        div {
            iframe {
                id: "map_iframe",
                width: "1024",
                height: "768",
                srcdoc: "{html}",
                style: "flex: 1;\
                    width: 100%;\
                    border: none;",
            }
        }
        div {
            style: "margin-top: 20px;",
            label { "Track Tail:" }
            input {
                r#type: "range",
                min: "1",
                max: "100",
                value: "{slider_value}",
                style: "width: 100%;",
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<i64>() {
                        slider_value.set(val);
                    }
                }
            }
            p {
                "Current slider value: {slider_value}"
            }
        }
    }
}

async fn update_tracker_data(amount: i64, mut trackers: Signal<Vec<SelectedTracker>>) {

    let mut updated = trackers.read().clone();

    for tracker in &mut updated {
        if tracker.tracker_id != DEFAULT_SELECTOR {

            let tracker_name = TRACKER_OPTIONS
                .iter()
                .find(|x| x.0 == tracker.tracker_id)
                .map(|x| x.1.to_string())
                .unwrap_or("Unknown".to_string());

            let payload = TrackerPayload {
                tracker_id: tracker.tracker_id.clone(),
                tracker_name,
                start_time: None,
                end_time: None,
            };
            let response = send_tracker_request_actual(payload,amount)
                .await
                .unwrap_or_default();
            tracker.data=response;

        }
    }
    trackers.set(updated);
}

fn add_tracker_trace(mut html: String, trackers: Signal<Vec<SelectedTracker>>) -> String {
    let trackers_data = trackers.read();

    for (index, tracker) in trackers_data.iter().enumerate() {
        if tracker.tracker_id != DEFAULT_SELECTOR {
            let mut coordinates = vec![];
            for coord in tracker.data.result.data.iter() {
                let coordinate = Coordinate {
                    lat: (coord.lat as f32) / 1000000.0,
                    lon: (coord.lon as f32) / 1000000.0,
                };
                let point = (coordinate.lat, coordinate.lon, tracker.tracker_id.as_str());
                coordinates.push(point);
            }
            let color = match index {
                1 => "red",
                2 => "green",
                3 => "yellow",
                4 => "orange",
                _ => "blue",
            };

            if !coordinates.is_empty() {
                // Generate polyline for all points (the "tail")
                let polyline_js = generate_polyline(&coordinates, color);
                
                // Generate a single marker for the latest position
                let latest_position = if let Some(latest) = coordinates.first() {
                    vec![*latest]
                } else {
                    vec![]
                };
                let marker_js = generate_markers(latest_position, color);
                
                // Combine polyline and marker JavaScript
                let combined_js = format!("{}\n{}", polyline_js, marker_js);
                
                if index == 0 {
                    html = html.replace("<!--BLUE_MARKERS-->", &combined_js);
                } else if index == 1 {
                    html = html.replace("<!--RED_MARKERS-->", &combined_js);
                }
            }
        }
    }
    html
}
