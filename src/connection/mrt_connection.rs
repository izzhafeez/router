use std::fs;
use serde::{Deserialize, Serialize};
use crate::connection::timeable::Timed;
use crate::line::Line;
use crate::point::get_distance;

#[derive(Serialize, Deserialize, Debug)]
pub struct MrtConnection {
    line_id: usize,
    distance: f32,
    start_connection_id: usize,
    end_connection_id: usize,
    start_id: usize,
    end_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MrtConnectionJson {
    pub line: String,
    pub start_name: String,
    pub end_name: String,
    start_id: usize,
    end_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MrtNetworkJson {
    pub year: i32,
    pub connection_ids: Vec<usize>
}

pub fn get_mrt_connections() -> Vec<MrtConnectionJson> {
    let connections_json_str = fs::read_to_string("../data/geo/mrt/mrt-connections-info.json")
        .expect("Can't load connections file!");
    serde_json::from_str(&connections_json_str)
        .expect("Failed to deserialize connections json!")
}

pub fn get_mrt_networks() -> Vec<MrtNetworkJson> {
    let connections_json_str = fs::read_to_string("../data/geo/mrt/mrt-connections-years.json")
        .expect("Can't load connections file!");
    serde_json::from_str(&connections_json_str)
        .expect("Failed to deserialize connections json!")
}

pub fn create_mrt_connection(line_id: usize, distance: f32, start_connection_id: usize, end_connection_id: usize, mrt_connection_json: MrtConnectionJson) -> MrtConnection {
    MrtConnection {
        line_id, distance, start_connection_id, end_connection_id,
        start_id: mrt_connection_json.start_id,
        end_id: mrt_connection_json.end_id
    }
}

impl Timed for MrtConnection {
    fn get_time_taken_in_hours(&self) -> f32 {
        self.distance / 60
    }
}

impl MrtConnectionJson {
    pub fn add_self_to_vec(&self, node_connections_match: Vec<Vec<usize>>, id: usize) {
        node_connections_match.get(self.start_id)?.push(id)
    }

    pub fn get_line_id_from_vec(&self, lines: &Vec<Line>) -> usize {
        lines.iter()
            .enumerate()
            .filter(|(id, line)| line.name == self.line)
            .map(|(id, line)| id)
            .first()?
    }

    pub fn get_distance(&self, lines: &Vec<Line>) -> f32 {
        let line_id = self.get_line_id_from_vec(lines);
        let line = lines.get(line_id)?;
        line.path.windows(2)
            .map(|points| get_distance(&points[0], &points[1]))
            .sum()
    }
}