use crate::utils::{
    average_geographic_position, generate_markers, send_tracker_request_actual, Coordinate,
};
use crate::{SelectedTracker, SliderValue, TrackerPayload, DEFAULT_SELECTOR, TRACKER_OPTIONS};
use dioxus::prelude::*;

#[component]
pub(crate) fn LiveMap() -> Element {
    let trackers = use_context::<Signal<Vec<SelectedTracker>>>();
    let trackers_data = use_signal(|| vec![]);
    let mut slider_value = use_signal(|| 1);
    let mut blue_markers = vec![];

    use_effect(move || {
        let trackers_clone = trackers.read().clone();
        let mut trackers_data = trackers_data.clone();
        let slider_value = slider_value.clone();
        let amount = slider_value.read().clone() + 1;

        spawn(async move {

            for (index, tracker) in trackers_clone.into_iter().enumerate() {
                if tracker.tracker_id != DEFAULT_SELECTOR {
                    println!("Selected order {}", index + 1);
                    println!("Tracker ID: {}", tracker.tracker_id);

                    let tracker_name = TRACKER_OPTIONS
                        .iter()
                        .find(|x| x.0 == tracker.tracker_id)
                        .map(|x| x.1.to_string())
                        .unwrap_or("Unknown".to_string());

                    println!("Tracker Name: {}", tracker_name);

                    let payload = TrackerPayload {
                        tracker_id: tracker.tracker_id.clone(),
                        tracker_name,
                        start_time: Some(1748303264),
                        end_time: Some(1748303264),
                    };
                    let response = send_tracker_request_actual(payload, amount)
                        .await
                        .unwrap_or_default();

                    let tracker_data = SelectedTracker {
                        tracker_id: tracker.tracker_id.clone(),
                        data: response,
                    };

                    trackers_data.write().push(tracker_data);
                }
            }
        });
    });

    let tracker_data_cloned = trackers_data.clone();

    for tracker in tracker_data_cloned.iter() {
        println!("{:?}", tracker.tracker_id);
        let cloned_tracker = tracker.clone();
        for coord in cloned_tracker.data.result.data.iter() {
            let coordinate = Coordinate {
                lat: (coord.lat as f32) / 1000000.0,
                lon: (coord.lon as f32) / 1000000.0,
            };

            let marker = (coordinate.lat, coordinate.lon, "Temporary placeholder");
            blue_markers.push(marker);
        }
    }

    let mut html = include_str!("../../static/assets/map_template.html").to_string();

    let all_coordinates = vec![];

    let mid = average_geographic_position(all_coordinates);
    let latitude = mid.lat;
    let longitude = mid.lon;
    html = html.replace("<!--START_LAT-->", &latitude.to_string());
    html = html.replace("<!--START_LON-->", &longitude.to_string());

    if blue_markers.len() > 0 {
        let marker_js: String = generate_markers(blue_markers.clone(), "blue");
        html = html.replace("<!--BLUE_MARKERS-->", &marker_js);
    }

    rsx! {
        div {
            iframe {
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
                min: "0",
                max: "100",
                value: "{slider_value}",
                style: "width: 100%;",
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<i64>() {
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
