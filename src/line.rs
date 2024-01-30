use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::point::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    pub name: String,
    pub path: Vec<Point>
}

pub fn get_lines() -> Vec<Line> {
    let lines_json_str = fs::read_to_string("../data/geo/mrt/mrt-lines-clean.json")
        .expect("Can't load lines file!");
    serde_json::from_str(&lines_json_str)
        .expect("Failed to deserialize lines json!")
}