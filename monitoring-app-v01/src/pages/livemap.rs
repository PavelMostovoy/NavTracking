use crate::utils::{average_geographic_position, generate_markers, generate_polyline, send_tracker_request_actual, Coordinate};
use crate::{MapDisplayState, SelectedTracker, TrackerPayload, DEFAULT_SELECTOR, TRACKER_OPTIONS};
use dioxus::prelude::*;
use std::time::Duration;
use dioxus_logger::tracing;
use tokio::time::sleep;

#[component]
pub(crate) fn LiveMap() -> Element {
    let trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let map_state = use_context::<Signal<MapDisplayState>>();


    let mut html = include_str!("../../static/assets/map_template.html").to_string();
    html = html.replace("<!--ZOOM_LEVEL-->", map_state.read().zoom.to_string().as_str());
    html = html.replace("<!--START_LAT-->", map_state.read().coordinate.lat.to_string().as_str());
    html = html.replace("<!--START_LON-->", map_state.read().coordinate.lon.to_string().as_str());

    // Potential bug with not updating a map if it is not moved
    use_coroutine(move |_: UnboundedReceiver<()>| {
        to_owned![trackers];
        async move {
            loop {
                update_tracker_data(100, trackers.clone()).await;
                tracing::info!("Updated by timer");
                sleep(Duration::from_secs(60)).await;
            }
        }
    });

    // Generate HTML with the current tracker data
    let html = use_memo(move || {
        let _ = trackers.read();
        html = add_tracker_trace(html.clone(), trackers, false);
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

fn add_tracker_trace(mut html: String, trackers: Signal<Vec<SelectedTracker>>, update_center: bool) -> String {
    let trackers_data = trackers.read();
    let mut all_coordinates:Vec<Coordinate> = vec![];

    for (index, tracker) in trackers_data.iter().enumerate() {
        if tracker.tracker_id != DEFAULT_SELECTOR {
            let mut coordinates = vec![];
            for coord in tracker.data.result.data.iter() {
                let coordinate = Coordinate {
                    lat: (coord.lat as f32) / 1000000.0,
                    lon: (coord.lon as f32) / 1000000.0,
                };
                all_coordinates.push(coordinate.clone());
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
    if update_center {
        let map_state = use_context::<Signal<MapDisplayState>>();
        let mid = average_geographic_position(all_coordinates);
        let latitude = mid.lat;
        let longitude = mid.lon;
        html = html.replace("<!--ZOOM_LEVEL-->", map_state.read().zoom.to_string().as_str());
        html = html.replace("<!--START_LAT-->", &latitude.to_string());
        html = html.replace("<!--START_LON-->", &longitude.to_string());
    }

    html
}
