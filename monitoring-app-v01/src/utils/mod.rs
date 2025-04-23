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