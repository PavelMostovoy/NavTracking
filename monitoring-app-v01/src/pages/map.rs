use crate::utils::{average_geographic_position, Coordinate};
use crate::TrackerResponse;
use dioxus::prelude::*;
use std::fs;

#[component]
pub(crate) fn MyMap(id: i32) -> Element {
    let context = use_context::<Signal<TrackerResponse>>();
    let tracker_data = context.read().clone();

    let mut slider_value = use_signal(|| 0);

    let memo_html = use_memo(move || {
        let mut markers = vec![];
        let mut coordinates = vec![];

        for coord in tracker_data.result.data.iter() {
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
                    r#"L.circleMarker([{lat}, {lon}], {{radius: 2, color: 'blue'}} ).addTo(map).bindPopup("{name}");"#,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut html =
            fs::read_to_string("assets/map_template.html").expect("can't read map template");

        html = html.replace("<!--START_LAT-->", &latitude.to_string());
        html = html.replace("<!--START_LON-->", &longitude.to_string());
        html = html.replace("<!--MARKERS-->", &marker_js);

        html
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