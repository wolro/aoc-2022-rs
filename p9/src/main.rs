/// Advent of Code day 9
/// https://adventofcode.com/2022/day/9

use itertools::Itertools;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    pos: Pos,
    prev_pos: Pos,
}

fn parse_moves(lines: Vec<&str>) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();
    for line in lines {
        let (dir, nr_str) = line.split_once(' ').expect("Error splitting input line.");
        let nr: usize = nr_str.parse().expect("Error parsing step number.");
        for _idx in 0..nr { moves.push(dir.to_owned()) };
    }
    moves
}

fn initialize_trackers() -> (Vec<Node>, Vec<Node>) {
    let mut head_tracker: Vec<Node> = Vec::new();
    let mut tail_tracker: Vec<Node> = Vec::new();
    let start_node: Node = Default::default();
    head_tracker.push(start_node.clone());
    tail_tracker.push(start_node);
    (head_tracker, tail_tracker)
}

fn update_nodes(head_tracker: &Vec<Node>, tail_tracker: &Vec<Node>,rope_move: String) -> (Node, Node) {
    let mut curr_head_node = *(&head_tracker.last().clone()).expect("Empty head tracker?");
    curr_head_node.prev_pos = curr_head_node.pos;
    let mut curr_tail_node = *(&tail_tracker.last().clone()).expect("Empty tail tracker?");
    curr_tail_node.prev_pos = curr_tail_node.pos;
    match rope_move.as_ref() {            
        "R" => { curr_head_node.pos.x += 1; },
        "L" => { curr_head_node.pos.x -= 1; },
        "U" => { curr_head_node.pos.y += 1; },
        "D" => { curr_head_node.pos.y -= 1; },
        _ => {println!("Faulty input move?")}
    }
    if (curr_tail_node.pos.x.abs_diff(curr_head_node.pos.x) > 1) || (curr_tail_node.pos.y.abs_diff(curr_head_node.pos.y) > 1)  {
        curr_tail_node.pos = curr_head_node.prev_pos;
        // dbg!(&curr_head_node.pos); dbg!(&curr_tail_node.pos);
    }
    (curr_head_node, curr_tail_node)
}

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    let (mut head_tracker, mut tail_tracker) = initialize_trackers();

    for rope_move in rope_moves {        
        let (curr_head_node, curr_tail_node) = update_nodes(&head_tracker, &tail_tracker, rope_move);
        head_tracker.push(curr_head_node);
        tail_tracker.push(curr_tail_node);
    }   

    let tail_pos: Vec<_> = tail_tracker.iter()
        .map(|ele| ele.pos)
        .unique()
        .collect();
    
    println!("The rope tail visited {} grid positions.", &tail_pos.len());
}


#[test]
fn test_tail_tracking_on_example_input_part1() {
    let lines = include_str!("../input_test.txt").lines().collect::<Vec<_>>();
    let rope_moves = parse_moves(lines);

    let (mut head_tracker, mut tail_tracker) = initialize_trackers();

    for rope_move in rope_moves {        
        let (curr_head_node, curr_tail_node) = update_nodes(&head_tracker, &tail_tracker, rope_move);
        head_tracker.push(curr_head_node);
        tail_tracker.push(curr_tail_node);
    }   

    let tail_pos: Vec<_> = tail_tracker.iter()
        .map(|ele| ele.pos)
        .unique()
        .collect();

    assert_eq!(tail_pos.len(), 13)
}

