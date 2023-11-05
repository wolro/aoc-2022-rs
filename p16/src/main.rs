/// Advent of Code day 16
/// https://adventofcode.com/2022/day/16
use std::time::Instant;

use anyhow::Result;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
/// The obligatory parser.
fn parse_input(lines: Vec<&str>) -> Result<(Vec<Point>, Vec<Point>)> {
    unimplemented!();
}

fn main() -> Result<()> {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();

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
