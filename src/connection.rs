mod bus_connection;
mod mrt_connection;
mod walk_connection;
mod timeable;

use std::fs;
use serde::{Deserialize, Serialize};
use crate::connection::bus_connection::{BusConnection, BusConnectionJson, get_bus_connections};
use crate::connection::mrt_connection::{get_mrt_connections, get_mrt_networks, create_mrt_connection, MrtConnection, MrtConnectionJson, MrtNetworkJson};
use crate::connection::walk_connection::WalkConnection;

#[derive(Serialize, Deserialize, Debug)]
pub enum Connection {
    Bus(BusConnection),
    Mrt(MrtConnection),
    Walk(WalkConnection)
}

pub fn get_networks_and_connections() -> (Vec<MrtNetworkJson>, Vec<MrtConnectionJson>, Vec<BusConnectionJson>) {
    let bus_connections = get_bus_connections();
    let mrt_connections = get_mrt_connections();
    let mrt_networks = get_mrt_networks();
    (mrt_networks, mrt_connections, bus_connections)
}