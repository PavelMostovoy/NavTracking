use chrono::{NaiveDate, TimeZone, Utc};
use reqwest::Client;
use serde::Deserialize;
use crate::{TrackerPayload, TrackerResponse};

#[derive(Debug, Clone)]
pub(crate) struct Coordinate {
    pub(crate) lat: f32,
    pub(crate) lon: f32,
}

pub(crate) fn average_geographic_position(coords: Vec<Coordinate>) -> Coordinate {
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
    let client = Client::new();
    let res = client
        .post("https://api.mostovoi.org/get_single_track")
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
