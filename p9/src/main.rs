/// Advent of Code day 9
/// https://adventofcode.com/2022/day/9
///
/// This one was awful and took way longer than it should have.
/// I quickly found a solution for part 1, which relied on the fact that for the 2-node
/// rope, following the head is equivalent to move the tail to the heads' previous position.
/// I couldn't make this wor for part 2 though (not sure if possible?)...
/// after much playing around, I broke down, hand-coded the cases and only could make
/// stuff work after a lot of debugging. Perhaps it would have been easier to follow the
/// advice and visualize the rope movement, but I don't think so, mostly I was dealing
/// with missing cases causing a panic anyway.
///
/// I ended up with very imperative code doing a lot of in-place vector mutation.
/// In the end I am fed up sufficiently with this puzzle that I didn't bother to spend
/// a lot of time cleaning the code up after finding a solution.

use itertools::Itertools; // for "unique()" iterator adaptor
use std::ops::Sub;

/// Position of a rope node.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}
/// Implements Substraction for our "Pos" struct. At least I learned how to
/// overload operators by implmeneting corresponding traits for this puzzle.
impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Represents a rope node. Yes, a struct with a single struct as field seems
/// quite useless. It previously had also a "previous position" state that
/// turned out useless in the end, and now I don't feel like cleaning this up.
#[derive(Debug, Default, Clone, Copy)]
struct Node {
    pos: Pos,
}

/// Parses input into instruction vector.
fn parse_moves(lines: Vec<&str>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();
    for line in lines {
        let (dir, nr_str) = line.split_once(' ').expect("Error splitting input line.");
        let nr: usize = nr_str.parse().expect("Error parsing step number.");
        for _idx in 0..nr {
            moves.push(dir.to_owned())
        }
    }
    moves
}

/// Builds vector containing "node_nr" nodes to represent a node.
fn initialize_rope(node_nr: u32) -> Vec<Node> {
    let mut rope_nodes: Vec<Node> = Vec::new();
    let node: Node = Default::default();
    for _idx in 0..node_nr {
        rope_nodes.push(node.clone());
    }
    rope_nodes
}

/// Moves head, then moves other nodes accordingly, using this horrible match statement.
fn update_nodes(rope_nodes: &mut Vec<Node>, rope_move: &str) {
    match rope_move.as_ref() {
        "R" => {
            rope_nodes[0].pos.x += 1;
        }
        "L" => {
            rope_nodes[0].pos.x -= 1;
        }
        "U" => {
            rope_nodes[0].pos.y += 1;
        }
        "D" => {
            rope_nodes[0].pos.y -= 1;
        }
        _ => {
            println!("Faulty input move?")
        }
    }

    for idx in 1..rope_nodes.len() {
        let node_diff = rope_nodes[idx - 1].pos - rope_nodes[idx].pos;
        match (node_diff.x, node_diff.y) {
            (0, 0) => {}
            (1, 0) | (0, 1) | (1, 1) => {}
            (2, 0) => {
                rope_nodes[idx].pos.x += 1;
            }
            (0, 2) => {
                rope_nodes[idx].pos.y += 1;
            }
            (2, 1) | (1, 2) => {
                rope_nodes[idx].pos.x += 1;
                rope_nodes[idx].pos.y += 1;
            }
            (-1, 0) | (0, -1) | (-1, -1) => {}
            (-2, 0) => {
                rope_nodes[idx].pos.x -= 1;
            }
            (0, -2) => {
                rope_nodes[idx].pos.y -= 1;
            }
            (-2, -1) | (-1, -2) => {
                rope_nodes[idx].pos.x -= 1;
                rope_nodes[idx].pos.y -= 1;
            }
            (1, -1) | (-1, 1) => {}
            (2, -1) | (1, -2) => {
                rope_nodes[idx].pos.x += 1;
                rope_nodes[idx].pos.y -= 1;
            }
            (-2, 1) | (-1, 2) => {
                rope_nodes[idx].pos.x -= 1;
                rope_nodes[idx].pos.y += 1;
            }
            (-2, -2) => {
                rope_nodes[idx].pos.x -= 1;
                rope_nodes[idx].pos.y -= 1;
            }
            (-2, 2) => {
                rope_nodes[idx].pos.x -= 1;
                rope_nodes[idx].pos.y += 1;
            }
            (2, -2) => {
                rope_nodes[idx].pos.x += 1;
                rope_nodes[idx].pos.y -= 1;
            }
            (2, 2) => {
                rope_nodes[idx].pos.x += 1;
                rope_nodes[idx].pos.y += 1;
            }
            _ => {
                dbg!(idx);
                dbg!(node_diff);
                panic! {"Something went wrong..."}
            }
        }
    }
}

/// Collects head (for debugging purposes) and tail positions in vectors,
/// filter for unique positions and return them.
fn simulate_rope_tail(rope_moves: &Vec<String>, mut rope_nodes: Vec<Node>) -> Vec<Pos> {
    let mut tail_tracker: Vec<Node> = Vec::new();
    let mut head_tracker: Vec<Node> = Vec::new();

    for rope_move in rope_moves {
        update_nodes(&mut rope_nodes, &rope_move);
        head_tracker.push(*rope_nodes.first().clone().unwrap());
        tail_tracker.push(*rope_nodes.last().clone().unwrap());
    }

    let tail_pos: Vec<_> = tail_tracker
        .iter()
        .map(|ele| ele.pos) // map node structures to their positions
        .unique()
        .collect();
    tail_pos
}

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    // part 1: only 2 nodes
    let rope_nodes = initialize_rope(2);
    let tail_pos = simulate_rope_tail(&rope_moves, rope_nodes);
    println!(
        "The 2-node rope tail visited {} grid positions.",
        &tail_pos.len()
    );

    // part 2: 10 nodes
    let rope_nodes = initialize_rope(10);
    let tail_pos = simulate_rope_tail(&rope_moves, rope_nodes);
    println!(
        "The 10-node rope tail visited {} grid positions.",
        &tail_pos.len()
    );
}

#[test]
fn test_tail_tracking_on_example_input_part1() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    let rope_nodes = initialize_rope(2);
    let tail_pos = simulate_rope_tail(&rope_moves, rope_nodes);
    assert_eq!(tail_pos.len(), 13)
}

#[test]
fn test_tail_tracking_on_example_input_part2() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    let rope_nodes = initialize_rope(10);
    let tail_pos = simulate_rope_tail(&rope_moves, rope_nodes);
    assert_eq!(tail_pos.len(), 1)
}

#[test]
fn test_tail_tracking_on_larger_example_input_part2() {
    let lines = include_str!("../input_test2.txt")
        .lines()
        .collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    let rope_nodes = initialize_rope(10);
    let tail_pos = simulate_rope_tail(&rope_moves, rope_nodes);
    assert_eq!(tail_pos.len(), 36)
}
