use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub lng: f32,
    pub lat: f32
}

pub fn get_distance(p1: &Point, p2: &Point) -> f32 {
    111.33 * f32::powf(f32::powi(p1.lat-p2.lat, 2) + f32::powi(p1.lng-p2.lng, 2), 0.5)
}