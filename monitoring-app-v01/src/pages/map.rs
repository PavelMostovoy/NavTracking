use crate::utils::{average_geographic_position, generate_markers, Coordinate};
use crate::{MapDisplayState, SelectedTracker, DEFAULT_SELECTOR};
use dioxus::prelude::*;

#[component]
pub(crate) fn MyMap(id: i32) -> Element {
    let trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let map_state = use_context::<Signal<MapDisplayState>>();

    let memo_html = use_memo(move || {
        let trackers_data = trackers.read();
        let mut html = include_str!("../../static/assets/map_template.html").to_string();

        let mut blue_markers = vec![];
        let mut red_markers = vec![];

        let mut all_coordinates = vec![];

        for (index, tracker) in trackers_data.iter().enumerate() {
            if !tracker.tracker_id.is_empty() && tracker.tracker_id != DEFAULT_SELECTOR {
                println!(
                    "Tracker id: {} tracker Data: {:?}",
                    tracker.tracker_id, tracker.data
                );

                for coord in tracker.data.result.data.iter() {
                    let coordinate = Coordinate {
                        lat: (coord.lat as f32) / 1000000.0,
                        lon: (coord.lon as f32) / 1000000.0,
                    };
                    all_coordinates.push(coordinate.clone());

                    let marker = (coordinate.lat, coordinate.lon, tracker.tracker_id.as_str());

                    if index == 0 {
                        blue_markers.push(marker);
                    }
                    if index == 1 {
                        red_markers.push(marker);
                    }
                }
            }
        }

        let mid = average_geographic_position(all_coordinates);
        let latitude = mid.lat;
        let longitude = mid.lon;

        html = html.replace("<!--ZOOM_LEVEL-->", map_state.read().zoom.to_string().as_str());
        html = html.replace("<!--START_LAT-->", &latitude.to_string());
        html = html.replace("<!--START_LON-->", &longitude.to_string());

        if blue_markers.len() > 0 {
            let marker_js: String = generate_markers(blue_markers.clone(), "blue");
            html = html.replace("<!--BLUE_MARKERS-->", &marker_js);
        }

        if red_markers.len() > 0 {
            let marker_js_additional: String = generate_markers(red_markers, "red");
            html = html.replace("<!--RED_MARKERS-->", &marker_js_additional);
        }
        html
    });

    rsx! {
            div {
                iframe {
                    width: "1024",
                    height: "768",
                    srcdoc: "{memo_html}",
                    style: "flex: 1;\
                        width: 100%;\
                        border: none;",
                }

            }
        }
}

