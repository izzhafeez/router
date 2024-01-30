use std::fs;
use serde::{Deserialize, Serialize};
use crate::connection::timeable::Timed;

#[derive(Serialize, Deserialize, Debug)]
pub struct BusConnection {
    service: String,
    distance: f32,
    start_id: usize,
    end_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusConnectionJson {
    service: String,
    start_name: String,
    end_name: String
}

pub fn get_bus_connections() -> Vec<BusConnectionJson> {
    let connections_json_str = fs::read_to_string("../data/geo/bus/bus-connections-list.json")
        .expect("Can't load connections file!");
    serde_json::from_str(&connections_json_str)
        .expect("Failed to deserialize connections json!")
}

impl Timed for BusConnection {
    fn get_time_taken_in_hours(&self) -> f32 {
        self.distance / 40;
    }
}