mod mrt_platform;
mod bus_stop;

use serde::{Deserialize, Serialize};
use crate::node::bus_stop::{BusStop, BusStopJson, get_bus_stops, create_bus_stop};
use crate::node::mrt_platform::{MrtPlatform, MrtPlatformJson, create_mrt_platform, get_mrt_platforms};

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    Bus(BusStop),
    Mrt(MrtPlatform)
}

pub fn get_nodes() -> (Vec<MrtPlatformJson>, Vec<BusStopJson>) {
    (get_mrt_platforms(), get_bus_stops())
}