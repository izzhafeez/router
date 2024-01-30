use std::fs;
use serde::{Deserialize, Serialize};
use crate::point::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct BusStop {
    label: String,
    road: String,
    desc: String,
    point: Point,
    connection_ids: Vec<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusStopJson {
    label: String,
    road: String,
    desc: String,
    lat: f32,
    lng: f32
}

pub fn get_bus_stops() -> Vec<BusStopJson> {
    let bus_stop_json_str = fs::read_to_string("../data/geo/bus/bus-stops-list.json")
        .expect("Can't load bus stop file!");
    serde_json::from_str(&bus_stop_json_str)
        .expect("Can't deserialize bus stop data!")
}

pub fn create_bus_stop(node: BusStopJson, connection_ids: Vec<usize>) -> BusStop {
    BusStop {
        label: node.label,
        road: node.road,
        desc: node.desc,
        point: Point { lat: node.lat, lng: node.lng },
        connection_ids
    }
}