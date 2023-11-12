/// Advent of Code day 16
/// https://adventofcode.com/2022/day/16
/// In case I refactor this into struct/impl I can untangle this very procedural piece of code :-)
///
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::time::Instant;

use anyhow::Result;
use itertools::Itertools;
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

fn main() -> Result<()> {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let valves = parse_input(lines)?;

    let before_part1 = Instant::now();
    let (graph, node_index_map) = build_graph(valves)?;

    export_graph(&graph)?;

    let shortest_distances = floyd_warshall::floyd_warshall(&graph, |_edge| 1).unwrap();

    let relevant_nodes: Vec<_> = graph
        .node_indices()
        .filter(|idx| (graph[*idx].flowrate > 0))
        .collect();

    let start_node = idx_by_name(&node_index_map, "AA");

    // generate population
    let pop_size: usize = 10000;
    let pop = generate_population(pop_size, relevant_nodes);

    // evaluate population
    let pressures: Vec<_> = pop
        .clone()
        .iter()
        .map(|member| {
            evaluate_member(
                start_node,
                member.to_owned(),
                graph.clone(),
                &shortest_distances,
            )
        })
        .collect();

    // assign fitness
    let mut idcs_sort = (0..pop_size).collect::<Vec<_>>();
    idcs_sort.sort_by_key(|&i| &pressures[i]);
    idcs_sort.reverse();

    let mut fitnesses: Vec<f64> = (0..pop_size)
        .map(|i| 2.0 / (pop_size as f64) * (1.0 - (i as f64 - 1.0) / (pop_size as f64 - 1.0)))
        .collect();

    let fitnesses_sorted: Vec<_> = idcs_sort.iter().map(|&i| fitnesses[i]).collect();

    dbg!(idcs_sort[0]);
    dbg!(idcs_sort.last().unwrap());
    dbg!(&pressures[idcs_sort[0]]);
    dbg!(&fitnesses_sorted[idcs_sort[0]]);

    let max_fitness = fitnesses.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    dbg!(&max_fitness);

    let (max_index, max_pressure) = &pressures
        .iter()
        .enumerate()
        .max_by_key(|(_, &val)| val)
        .unwrap();
    dbg!(&max_index);
    dbg!(&max_pressure);

    println!("Part 1:");
    println!("Elapsed time: {:.2?}", before_part1.elapsed());

    let before_part2 = Instant::now();
    println!("Part 2:");
    println!("Elapsed time: {:.2?}", before_part2.elapsed());

    Ok(())
}

fn generate_population(pop_size: usize, relevant_nodes: Vec<NodeIndex>) -> Vec<Vec<NodeIndex>> {
    let pop: Vec<Vec<NodeIndex>> = (0..pop_size)
        .map(|_| {
            let mut member = relevant_nodes.clone();
            member.shuffle(&mut thread_rng());
            member
        })
        .collect();
    pop
}

fn evaluate_member(
    start_node: NodeIndex,
    mut pop_member: Vec<NodeIndex>,
    mut graph: Graph<Valve, u32, Undirected>,
    shortest_distances: &HashMap<(NodeIndex, NodeIndex), i32>,
) -> u32 {
    pop_member.insert(0, start_node);
    let mut time = 0;
    let mut released_pressure = 0;
    let mut flow_rates: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    for reset_idx in graph.node_indices() {
        graph[reset_idx].valve_open = false;
    }

    for i in 0..(pop_member.len() - 1) {
        let start_idx = pop_member[i];
        let target_idx = pop_member[i + 1];

        for _t in 0..shortest_distances[&(start_idx, target_idx)] {
            time += 1;
            let rel_p_permin: u32 = flow_rates.clone().iter().sum();
            released_pressure += rel_p_permin;
        }

        // open valve, or not
        if !(graph[target_idx].valve_open || graph[target_idx].flowrate == 0) {
            let max_flowrate = get_max_flowrate(&graph);

            let open_threshold = 1.0 - (graph[target_idx].flowrate as f64) / (max_flowrate as f64);

            let open_rng: f64 = rng.gen();
            if open_rng > open_threshold {
                time += 1;
                // Update released pressure
                let rel_p_permin: u32 = flow_rates.clone().iter().sum();
                released_pressure += rel_p_permin;
                flow_rates.push(graph[target_idx].flowrate);

                graph[target_idx].valve_open = true;
            }
        }
    }

    let rel_p_permin: u32 = flow_rates.clone().iter().sum();
    while time < 30 {
        released_pressure += rel_p_permin;
        time += 1;
    }

    released_pressure
}
