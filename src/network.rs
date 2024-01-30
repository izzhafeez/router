use serde::{Deserialize, Serialize};
use crate::connection::{Connection, get_networks_and_connections, create_mrt_connection, MrtConnectionJson, MrtConnection, MrtNetworkJson, BusConnectionJson};
use crate::line::{get_lines, Line};
use crate::node::{MrtPlatform, Node, MrtPlatformJson, create_mrt_platform, get_nodes, BusStop, BusStopJson, create_bus_stop};

#[derive(Serialize, Deserialize, Debug)]
pub struct Infrastructure {
    lines: Vec<Line>,
    nodes: Vec<Node>,
    connections: Vec<Connection>
}

#[derive(Serialize, Deserialize, Debug)]
struct Network {
    year: i32,
    lines: Vec<usize>,
    nodes: Vec<usize>,
    connections: Vec<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkJson {
    year: i32,
    mrt_connection_ids: Vec<usize>,
    bus_connection_ids: Vec<usize>
}

impl Infrastructure {
    fn get_node(&self, node_id: usize) -> &Node {
        self.nodes.get(node_id).expect("Node not found!")
    }

    fn get_line(&self, line_id: usize) -> &Line {
        self.lines.get(line_id).expect("Line not found!")
    }

    fn get_connection(&self, connection_id: usize) -> &Connection {
        self.connections.get(connection_id).expect("Connection not found!")
    }

    fn print_connection(&self, connection_id: usize) {
        let connection: &Connection = self.get_connection(connection_id);
        let start_node: &Node = self.get_node(connection.get_start_index());
        let end_node: &Node = self.get_node(connection.get_end_index());
        println!("<FROM {:?} TO {:?} VIA {:?}>", start_node, end_node, connection);
    }
}

pub fn create_infrastructure() -> Network {
    let (mrt_networks_json, mrt_connections_json, bus_connections_json) = get_networks_and_connections();
    let (mrt_nodes_json, bus_nodes_json) = get_nodes();
    let lines: Vec<Line> = get_lines();
    let mrt_connection_count = mrt_connections_json.len();
    let mrt_node_count = mrt_nodes_json.len();
    let bus_connection_count = bus_connections_json.len();
    let bus_node_count = bus_nodes_json.len();
    let mrt_networks = get_networks_json(mrt_networks_json, bus_connection_count);
    let mrt_nodes = create_mrt_nodes(mrt_nodes_json, &mrt_connections_json);
    let mrt_connections = create_mrt_connections(mrt_connections_json, &lines, &mrt_nodes);
}

fn get_networks_json(mrt_networks: Vec<MrtNetworkJson>, bus_connection_count: usize) -> Vec<NetworkJson> {
    mrt_networks.into_iter()
        .map(|network| NetworkJson {
            year: network.year,
            mrt_connection_ids: network.connection_ids,
            bus_connection_ids: (0..bus_connection_count).collect()
        })
        .collect()
}

fn create_mrt_nodes(mrt_nodes_json: Vec<MrtPlatformJson>, mrt_connections_json: &Vec<MrtConnectionJson>) -> Vec<MrtPlatform> {
    let node_connections_match: Vec<Vec<usize>> = Vec::with_capacity(mrt_nodes_json.len());
    mrt_connections_json.iter().enumerate()
        .for_each(|(id, connection)| connection.add_self_to_vec(node_connections_match, id));
    mrt_nodes_json.into_iter()
        .zip(node_connections_match.into_iter())
        .map(create_mrt_platform)
        .collect()
}

fn create_mrt_connections(mrt_connections_json: Vec<MrtConnectionJson>, lines: &Vec<Line>, mrt_nodes: &Vec<MrtPlatform>) -> Vec<MrtConnection> {
    mrt_connections_json.into_iter()
        .map(|connection| {
            let line_id = lines.iter().filter(|line| line.name == connection.line).first()?;
            let distance = connection.get_distance(lines);
            let start_connection_id = mrt_nodes.iter().filter(|node| node.label == connection.start_name).first()?;
            let end_connection_id = mrt_nodes.iter().filter(|node| node.label == connection.end_name).first()?;
            create_mrt_connection(line_id, distance, start_connection_id, end_connection_id, connection)
        })
        .collect()
}

fn create_bus_nodes(bus_nodes_json: Vec<BusStopJson>, bus_connections_json: &Vec<BusConnectionJson>, mrt_connection_count: usize) -> Vec<BusStop> {
    let node_connections_match: Vec<Vec<usize>> = Vec::with_capacity(bus_nodes_json.len());
    bus_connections_json.iter().enumerate()
        .for_each(|(id, connection)| connection.add_self_to_vec(node_connections_match, id));
    bus_nodes_json.into_iter()
        .zip(node_connections_match.into_iter())
        .map(create_bus_stop)
        .collect()
}