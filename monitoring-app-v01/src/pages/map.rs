use std::fs;
use dioxus::prelude::*;
use crate::TrackerResponse;
use crate::utils:: {Coordinate, average_geographic_position};


#[component]
pub(crate) fn MyMap(id: i32) -> Element {

    let mut markers = vec![];
    let mut coordinates = vec![];
    
    let context = use_context::<Signal<TrackerResponse>>();
    
    for coord in context().result.data {
        let coordinate = Coordinate {lat: (coord.lat as f32)/1000000.0 ,lon: (coord.lon as f32)/1000000.0};
        coordinates.push(coordinate.clone());
        let marker = (coordinate.lat, coordinate.lon, "Маркер A");
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
                lat = lat,
                lon = lon,
                name = name
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut html = fs::read_to_string("assets/map_template.html").expect("can't read map template");

    html = html.replace("<!--START_LAT-->", &latitude.to_string());
    html = html.replace("<!--START_LON-->", &longitude.to_string());
    html = html.replace("<!--MARKERS-->", &marker_js);

    rsx! {
        iframe {
            width: "800",
            height: "600",
            srcdoc: "{html}",
            style: "flex: 1;\
             width: 100%;\
              // height: 100%;\
               border: none;",
        }
    }
}