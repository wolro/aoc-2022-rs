/// Advent of Code day 16
/// https://adventofcode.com/2022/day/16
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::time::Instant;

use anyhow::Result;
use petgraph::algo::floyd_warshall;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::prelude::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    valvename: String,
    flowrate: u32,
    connections: String,
    valve_open: bool,
}

/// The obligatory parser.
fn parse_input(lines: Vec<&str>) -> Result<Vec<Valve>> {
    let mut valves: Vec<Valve> = Vec::new();

    for line in lines {
        let (valve_entry, conn_entry) = line
            .split_once(';')
            .expect("Input line didn't contain colon?");
        let vname_borders = (
            valve_entry.find("Valve ").unwrap() + 6,
            valve_entry.find("Valve ").unwrap() + 8,
        );
        let frate_border = valve_entry.find('=').unwrap() + 1;

        let mut conn_border: usize = 0;
        if let Some(entry) = conn_entry.find("valves ") {
            conn_border = entry + 7;
        } else {
            conn_border = conn_entry.find("valve ").unwrap() + 6;
        }

        valves.push(Valve {
            valvename: valve_entry[vname_borders.0..vname_borders.1].to_owned(),
            flowrate: valve_entry[frate_border..].parse::<u32>()?,
            connections: conn_entry[conn_border..].to_owned(),
            valve_open: false,
        });
    }

    Ok(valves)
}

fn idx_by_name(node_index_map: &HashMap<String, NodeIndex>, valve_name: &str) -> NodeIndex {
    *node_index_map.get(valve_name).unwrap()
}

fn export_graph(graph: &Graph<Valve, u32, Undirected>) -> Result<(), anyhow::Error> {
    let graphviz_rep = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut file_handle = File::create("test_graph.dot").unwrap();
    file_handle.write_all(graphviz_rep.as_bytes())?;
    Ok(())
}

fn build_graph(
    valves: Vec<Valve>,
) -> Result<(Graph<Valve, u32, Undirected>, HashMap<String, NodeIndex>)> {
    let mut graph: UnGraph<Valve, u32, _> = UnGraph::new_undirected();

    for valve in valves {
        graph.add_node(valve);
    }

    let node_index_map: HashMap<String, NodeIndex> = graph
        .node_indices()
        .map(|node_idx| (graph[node_idx].valvename.clone(), node_idx))
        .collect();

    for node_index in 0..graph.node_count() {
        let node_idx = NodeIndex::new(node_index);
        let conn_string = graph[node_idx].connections.clone();
        let conns = conn_string.split(", ").collect::<Vec<_>>();

        for destination in conns.iter() {
            graph.add_edge(
                node_idx,
                *node_index_map
                    .get(*destination)
                    .expect("Error adding edge, issue with node name?"),
                1,
            );
        }
    }

    Ok((graph, node_index_map))
}

/// In case I refactor this into struct/impl I can untangle this very procedural piece of code :-)
fn valvewalk_mc(
    mut graph: Graph<Valve, u32, Undirected>,
    node_index_map: HashMap<String, NodeIndex>,
) -> (u32, Vec<String>) {
    // reset variables
    let mut released_pressure = 0;
    let mut path: Vec<String> = Vec::new();
    let mut current_node = String::from("AA");
    let mut flow_rates: Vec<u32> = Vec::new();
    let mut time: u8 = 0;
    let mut rng = rand::thread_rng();
    for reset_idx in graph.node_indices() {
        graph[reset_idx].valve_open = false;
    }

    loop {
        let idx = idx_by_name(&node_index_map, current_node.as_str());
        path.push(current_node.to_owned());

        // open valve, or not
        if !(graph[idx].valve_open || graph[idx].flowrate == 0) {
            let max_flowrate = get_max_flowrate(&graph);

            let open_threshold = 1.0 - (graph[idx].flowrate as f64) / (max_flowrate as f64);
            // let open_threshold = 0.5;
            let open_rng: f64 = rng.gen();
            if open_rng > open_threshold {
                graph[idx].valve_open = true;

                flow_rates.push(graph[idx].flowrate);
                path.push(String::from("Valve opened."));
                // Update released pressure
                for rate in &flow_rates {
                    released_pressure += *rate;
                }
                time += 1;
            }
        }

        if time >= 29 {
            break;
        }

        // pick tunnel and move to next valve, or not
        let conn_string = graph[idx].connections.clone();
        let conns = conn_string
            .split(", ")
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();
        let path_rng: f32 = rng.gen();
        let path_choices = conns.len() - 1;
        let path_coin = (path_rng * (path_choices as f32)).round() as usize;

        current_node = conns[path_coin].clone();
        time += 1;

        // Update released pressure
        for rate in &flow_rates {
            released_pressure += *rate;
        }

        if time >= 29 {
            break;
        }
    }

    (released_pressure, path)
}

/// Return maximum available flow rate (corresponding to valves not already opened)
/// used for weighting of coin flip to open valve or not
fn get_max_flowrate(graph: &Graph<Valve, u32, Undirected>) -> u32 {
    let mut max_flowrate = 0;
    for rate_idx in graph.node_indices() {
        max_flowrate = if (graph[rate_idx].flowrate > max_flowrate) && !graph[rate_idx].valve_open {
            graph[rate_idx].flowrate
        } else {
            max_flowrate
        };
    }
    max_flowrate
}

// fn node_by_name(
//     node_index_map: &HashMap<String, NodeIndex>,
//     valve_name: &str,
//     graph: &Graph<Valve, u32, Undirected>,
// ) -> Option<Valve> {
//     node_index_map
//         .get(valve_name)
//         .map(|idx| graph[*idx].clone())
// }

// fn custom_traversal_order(
//     node_index_map: &HashMap<String, NodeIndex>,
//     visited_nodes: Vec<&str>,
// ) -> Vec<NodeIndex> {
//     let mut traversal_order = Vec::new();

//     for &node_name in &visited_nodes {
//         if let Some(node_idx) = node_index_map.get(node_name) {
//             traversal_order.push(*node_idx);
//         }
//     }

//     traversal_order
// }

fn main() -> Result<()> {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let valves = parse_input(lines)?;

    let before_part1 = Instant::now();
    let (mut graph, node_index_map) = build_graph(valves)?;

    export_graph(&graph)?;

    let shortest_distances = floyd_warshall::floyd_warshall(&graph, |_edge| 1).unwrap();

    let relevant_nodes_ini: Vec<_> = graph
        .node_indices()
        .filter(|idx| (graph[*idx].flowrate > 0))
        .collect();

    let mut relevant_nodes = relevant_nodes_ini.clone();

    let mut relevant_node_names: Vec<_> = relevant_nodes
        .clone()
        .iter()
        .map(|idx| graph[*idx].valvename.clone())
        .collect();

    dbg!(&relevant_node_names);

    let mut pressures: Vec<u32> = Vec::new();
    let mut paths: Vec<Vec<String>> = Vec::new();
    let start_node = idx_by_name(&node_index_map, "AA");

    for loop_idx in 0..1000000 {
        relevant_nodes.shuffle(&mut thread_rng());

        let mut loop_nodes = relevant_nodes.clone();
        loop_nodes.insert(0, start_node);
        let mut time = 0;
        let mut released_pressure = 0;
        let mut flow_rates: Vec<u32> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut path: Vec<String> = Vec::new();
        for reset_idx in graph.node_indices() {
            graph[reset_idx].valve_open = false;
        }

        'outer: loop {
            for i in 0..(loop_nodes.len() - 1) {
                let start_idx = loop_nodes[i];
                let target_idx = loop_nodes[i + 1];

                for _t in 0..shortest_distances[&(start_idx, target_idx)] {
                    time += 1;
                    let rel_p_permin: u32 = flow_rates.clone().iter().sum();
                    released_pressure += rel_p_permin;
                    // for rate in &flow_rates {
                    //     released_pressure += *rate;
                    // }
                    path.push(format!("Minute {}", &time));
                    path.push(format!("Releasing {} pressure.", &rel_p_permin));
                    if time >= 30 {
                        break 'outer;
                    }
                }

                path.push(format!(
                    "Arrived at valve {}. ",
                    graph[target_idx].valvename.clone()
                ));

                // open valve, or not
                if !(graph[target_idx].valve_open || graph[target_idx].flowrate == 0) {
                    let max_flowrate = get_max_flowrate(&graph);

                    let open_threshold =
                        1.0 - (graph[target_idx].flowrate as f64) / (max_flowrate as f64);
                    // let open_threshold = 0.5;
                    let open_rng: f64 = rng.gen();
                    if open_rng > open_threshold {
                        time += 1;
                        // Update released pressure
                        let rel_p_permin: u32 = flow_rates.clone().iter().sum();
                        released_pressure += rel_p_permin;
                        flow_rates.push(graph[target_idx].flowrate);
                        path.push(format!("Minute {}", &time));
                        path.push(format!("Releasing {} pressure.", &rel_p_permin));

                        graph[target_idx].valve_open = true;
                        path.push(String::from("Valve opened."));
                        // for rate in &flow_rates {
                        //     released_pressure += *rate;
                        // }

                        if time >= 30 {
                            break 'outer;
                        }
                    }
                }
            }
        }
        pressures.push(released_pressure);
        paths.push(path);
    }

    let (max_index, max_pressure) = &pressures
        .iter()
        .enumerate()
        .max_by_key(|(_, &val)| val)
        .unwrap();
    dbg!(&max_index);
    dbg!(&max_pressure);
    dbg!(&paths[*max_index]);
    // dbg!(&paths[max_index]);

    // let pressures: Vec<u32> = (0..1000000)
    //     .into_par_iter()
    //     .map(|_rep_idx| valvewalk_mc(graph.clone(), node_index_map.clone()).0)
    //     .collect();

    // dbg!(pressures.iter().max().unwrap());

    println!("Part 1:");
    println!("Elapsed time: {:.2?}", before_part1.elapsed());

    let before_part2 = Instant::now();
    println!("Part 2:");
    println!("Elapsed time: {:.2?}", before_part2.elapsed());

    Ok(())
}

#[test]
fn part1_validate_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    unimplemented!();
}

#[test]
fn part2_validate_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    unimplemented!();
}
