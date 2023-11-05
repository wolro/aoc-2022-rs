/// Advent of Code day 16
/// https://adventofcode.com/2022/day/16
use std::time::Instant;

use anyhow::Result;
// use petgraph::graph::UnGraph;
// use petgraph::prelude::*;
// use std::collections::HashMap;

struct NodeData {
    valvename: String,
    flowrate: u32,
}

#[derive(Default, Debug)]
struct Valve {
    valvename: String,
    flowrate: u32,
    connections: String,
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
        }
        else {
            conn_border = conn_entry.find("valve ").unwrap() + 6;
        }

        valves.push(Valve {
            valvename: valve_entry[vname_borders.0..vname_borders.1].to_owned(),
            flowrate: valve_entry[frate_border..].parse::<u32>()?,
            connections: conn_entry[conn_border..].to_owned(),
        });
    }

    Ok(valves)
}

fn main() -> Result<()> {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    dbg!(&lines);
    let valves = parse_input(lines);
    dbg!(&valves);

    let before_part1 = Instant::now();
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
