use crate::utils::{average_geographic_position, generate_markers, Coordinate};
use crate::{MapDisplayState, SelectedTracker, DEFAULT_SELECTOR};
use dioxus::prelude::*;
use dioxus_logger::tracing;

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
                tracing::info!(
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

        let tracker_placeholder :String = String::from("const sampleData = [
            {lat: 42.684734, lng: 3.034418, time: 1502529980, dir: 320, info: [{key: 'name', value: 'ship1'}]},
            {lat: 42.685734, lng: 3.035418, time: 1502531980, dir: 330, info: [{key: 'name', value: 'ship1'}]},
            {lat: 42.686734, lng: 3.036418, time: 1502532980, dir: 340, info: [{key: 'name', value: 'ship1'}]}
        ];

        const trackplayback = L.trackplayback(sampleData, map);
        const trackplaybackControl = L.trackplaybackcontrol(trackplayback);
        trackplaybackControl.addTo(map);");

        html = html.replace("<!--TRACKERS-PLAYBACK-->", &tracker_placeholder);


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

