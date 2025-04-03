use crate::lora_data::received_data::GeoData;

use base64::{Engine as _, engine::{general_purpose}};
use chrono::{DateTime, Utc};

fn bytes_to_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn decode_unix_timestamp(timestamp: u32) -> String {
    let datetime = DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());

    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

fn bytes_to_f32(bytes: &[u8]) -> f32 {
    f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

pub fn string_to_data(value: String) -> GeoData {
    let received_data = general_purpose::STANDARD.decode(value).expect("error decoding geodata");

    if received_data.len() != 12 {
        panic!("Incorrect Data Length {}", received_data.len());
    }

    let timestamp = bytes_to_u32(&received_data[0..4]);
    let lat = bytes_to_f32(&received_data[4..8]);
    let lon = bytes_to_f32(&received_data[8..12]);

    // println!("Raw Timestamp: {}", timestamp);
    // println!("Human-readable Time: {}", decode_unix_timestamp(timestamp));
    // println!("Latitude: {}, Longitude: {}", lat, lon);

    GeoData {
        time: timestamp,
        latitude: lat,
        longitude: lon,
    }
}

pub fn string_to_timestamp(value: &str) -> u32 {
    let datetime = DateTime::parse_from_rfc3339(value)
        .expect("Invalid date format")
        .with_timezone(&Utc);
    datetime.timestamp() as u32
}