use serde::{Deserialize, Serialize};
use crate::connection::timeable::Timed;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalkConnection {
    distance: f32,
    start_id: usize,
    end_id: usize
}

impl Timed for WalkConnection {
    fn get_time_taken_in_hours(&self) -> f32 {
        self.distance / 4
    }
}