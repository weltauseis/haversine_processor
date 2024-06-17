use std::io::Read;

use crate::json::JSONValue;

// NOTE(casey): EarthRadius is generally expected to be 6372.8
fn reference_haversine(x0: f64, y0: f64, x1: f64, y1: f64, earth_radius: f64) -> f64 {
    let mut lat1 = y0;
    let mut lat2 = y1;
    let lon1 = x0;
    let lon2 = x1;

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    lat1 = lat1.to_radians();
    lat2 = lat2.to_radians();

    let a = (d_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    return earth_radius * c;
}

pub fn calculate_average_haversine(pairs: Vec<(f64, f64, f64, f64)>) -> f64 {
    let mut accumulator: Vec<f64> = Vec::new();

    for (x0, y0, x1, y1) in pairs {
        accumulator.push(reference_haversine(x0, y0, x1, y1, 6372.8));
    }

    return accumulator.iter().sum::<f64>() / accumulator.len() as f64;
}

pub fn calculate_difference(average: f64, reference_path: &str) -> (f64, f64) {
    let mut references_file = std::fs::File::open(reference_path).unwrap();
    let mut bytes: [u8; 8] = [0; 8];

    let mut distances = Vec::new();
    loop {
        let nb_read = references_file.read(&mut bytes).unwrap();
        if nb_read < 8 {
            break;
        }

        let dist = f64::from_le_bytes(bytes);
        distances.push(dist);
    }

    let reference_average = distances.iter().sum::<f64>() / distances.len() as f64;
    return (reference_average, average - reference_average);
}
