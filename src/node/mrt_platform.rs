use std::fs;
use serde::{Deserialize, Serialize};
use crate::point::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct MrtPlatform {
    name: String,
    pub label: String,
    point: Point,
    connection_ids: Vec<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MrtPlatformJson {
    name: String,
    label: String,
    lat: f32,
    lng: f32
}

pub fn get_mrt_platforms() -> Vec<MrtPlatformJson> {
    let platform_json_str = fs::read_to_string("../data/geo/mrt/mrt-platform-list.json")
        .expect("Can't load platform file!");
    serde_json::from_str(&platform_json_str)
        .expect("Can't deserialize platform data!")
}

pub fn create_mrt_platform(node: MrtPlatformJson, connection_ids: Vec<usize>) -> MrtPlatform {
    MrtPlatform {
        name: node.name,
        label: node.label,
        point: Point { lat: node.lat, lng: node.lng },
        connection_ids
    }
}