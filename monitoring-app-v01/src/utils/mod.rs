use chrono::{NaiveDate, TimeZone, Utc};
use dioxus_logger::tracing;
use reqwest::Client;
use serde::Deserialize;
use crate::{TrackerPayload, TrackerResponse, TrackerResult};
use crate::config::Settings;


#[derive(Debug, Clone)]
pub(crate) struct Coordinate {
    pub(crate) lat: f32,
    pub(crate) lon: f32,
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate {
            lat: 42.7,
            lon: 3.03,
        }
    }
}




pub(crate) fn average_geographic_position(coords: Vec<Coordinate>) -> Coordinate {
    if coords.is_empty() {
        return Coordinate::default();
    }

    let (sum_lat, sum_lon) = coords.iter().fold((0.0, 0.0), |(sum_lat, sum_lon), coord| {
        (sum_lat + coord.lat, sum_lon + coord.lon)
    });

    let n = coords.len() as f32;
    Coordinate {
        lat: sum_lat / n,
        lon: sum_lon / n,
    }
}

pub fn date_to_unix_range(date: NaiveDate) -> Option<(i64, i64)> {
    let start_of_day = date.and_hms_opt(0, 0, 0)?;
    let end_of_day = date.and_hms_opt(23, 59, 59)?;

    let start_timestamp = Utc.from_utc_datetime(&start_of_day).timestamp();
    let end_timestamp = Utc.from_utc_datetime(&end_of_day).timestamp();

    Some((start_timestamp, end_timestamp))
}


pub async fn send_tracker_request(
    tracker_payload: TrackerPayload,
) -> Result<TrackerResponse, reqwest::Error> {
    let config = Settings::load();
    let client = Client::new();
    let endpoint = format!("{}/get_single_track", config.tracker_api_url);
    let res = client
        .post(endpoint)
        .json(&tracker_payload)
        .send()
        .await?;

    let tracker_data = res.json::<TrackerResponse>().await?;
    Ok(tracker_data)
}
pub async fn send_tracker_request_actual(
    tracker_payload: TrackerPayload,
    amount: i64
) -> Result<TrackerResponse, reqwest::Error> {
    let config = Settings::load();
    let client = Client::new();
    let endpoint = format!("{}/get_last_positions/{}", config.tracker_api_url, amount);
    tracing::info!("{:?}", tracker_payload);
    let res = client
        .post(endpoint)
        .json(&tracker_payload)
        .send()
        .await?;

    let tracker_data = res.json::<TrackerResponse>().await?;
    Ok(tracker_data)
}


#[derive(Debug, Deserialize, Clone)]
pub struct SimplifiedData {
    pub lat: i32,
    pub lon: i32,
    pub time: u32,
}

pub(crate) fn generate_markers(coordinates: Vec<(f32, f32, &str)>, color: &str) -> String {
    coordinates
        .iter()
        .map(|(lat, lon, name)| {
            format!(
                r#"
                (function() {{
                    const marker = L.circleMarker([{lat}, {lon}], {{
                        radius: 5,
                        color: '{color}',
                        fillColor: '{color}',
                        fillOpacity: 0.6,
                        weight: 2,
                        opacity: 0.8
                    }});

                    marker.bindPopup(`
                        <div style="font-family: 'Roboto', sans-serif; padding: 5px;">
                            <strong style="color: #333; font-size: 14px;">{name}</strong>
                            <div style="margin-top: 5px; font-size: 12px; color: #666;">
                                <div>Lat: {lat}</div>
                                <div>Lon: {lon}</div>
                            </div>
                        </div>
                    `, {{
                        className: 'custom-popup'
                    }});

                    marker.on('mouseover', function() {{
                        this.setStyle({{ radius: 7, weight: 3 }});
                    }});

                    marker.on('mouseout', function() {{
                        this.setStyle({{ radius: 5, weight: 2 }});
                    }});

                    marker.addTo(map);
                }})();
                "#,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
