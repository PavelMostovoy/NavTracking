use crate::utils::{average_geographic_position, Coordinate, SimplifiedData};
use crate::{utils, SelectedTracker, TrackerResponse};
use dioxus::prelude::*;
use std::fs;

#[component]
pub(crate) fn MyMap(id: i32) -> Element {
    let mut html = include_str!("../../static/assets/map_template.html").to_string();
    let trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let mut slider_value = use_signal(|| 0);
    
    let mut results_data: Vec<SimplifiedData>  = Vec::new();
    let mut color = "green";
    for (index, tracker) in trackers.read().clone().iter().enumerate() {
        
        if (tracker.tracker_id != ""){
            println!("Tracker id: {} tracker Data: {:?}",tracker.tracker_id, tracker.data);
            results_data = tracker.data.result.data.clone();
            if index == 0 {
                color = "blue";
            }
            if index == 1 {
                color = "red";
            }
        }
        
    }
    

    let memo_html = use_memo(move || {
        let mut markers = vec![];
        let mut coordinates = vec![];

        for coord in results_data.iter() {
            let coordinate = Coordinate {
                lat: (coord.lat as f32) / 1000000.0,
                lon: (coord.lon as f32) / 1000000.0,
            };
            coordinates.push(coordinate.clone());
            let marker = (coordinate.lat, coordinate.lon, "Marker A");
            markers.push(marker);
        }

        let mid = average_geographic_position(coordinates);
        let latitude = mid.lat;
        let longitude = mid.lon;

        let marker_js: String = markers
            .iter()
            .map(|(lat, lon, name)| {
                format!(
                    r#"L.circleMarker([{lat}, {lon}], {{radius: 2, color: '{color}'}} ).addTo(map).bindPopup("{name}");"#,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        html = html.replace("<!--START_LAT-->", &latitude.to_string());
        html = html.replace("<!--START_LON-->", &longitude.to_string());
        html = html.replace("<!--MARKERS-->", &marker_js);

        html.clone()
    });

    rsx! {
        div {
            iframe {
                width: "800",
                height: "600",
                srcdoc: "{memo_html}",
                style: "flex: 1;\
                    width: 100%;\
                    border: none;",
            }
            
            div {
                style: "margin-top: 20px;",
                label { "Slider (placeholder):" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "100",
                    value: "{slider_value}",
                    style: "width: 100%;",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<i32>() {
                            slider_value.set(val);
                            println!("Slider moved to: {val}");
                        }
                    }
                }
                p {
                    "Current slider value: {slider_value}"
                }
            }
        }
    }
}