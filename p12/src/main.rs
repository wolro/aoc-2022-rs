/// Advent of Code day 12
/// https://adventofcode.com/2022/day/12
///
/// A pathfinding problem with weights and a few special rules on what counts as obstacle.
/// I don't feel like re-implementing Dijkstra, so we will be lazy here.
///
/// First, looking for a suitable library results in the "pathfinding" crate,
/// which seems to be exactly what we need. In addition, there exists an article
/// on the topic, together with a Github repo with examples making use of the
/// "pathfinding" crate. Here, we will also snitch a few things, specifically
/// the "Board" structure:
/// https://blog.logrocket.com/pathfinding-rust-tutorial-examples/
/// https://github.com/gregstoll/rust-pathfinding

use pathfinding::prelude::dijkstra;

/// Structure encoding the position on the graph, with required derives as outlined here:
/// https://docs.rs/pathfinding/latest/pathfinding/directed/dijkstra/fn.dijkstra.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i16, i16);

/// "Successors" (or neightbours of the evaluated grid position) have a position
/// and a cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
struct Successor {
    pos: Pos,
    cost: u32,
}

/// Structure representing a rectangular grid or "board". Inspired by
/// https://github.com/gregstoll/rust-pathfinding/blob/main/src/lib.rs,
/// but adapted for the problem.
#[derive(Debug)]
struct Board {
    width: u16,
    height: u16,
    data: Vec<Vec<Option<u32>>>,
}
impl Board {
    fn new(board_lines: Vec<&str>) -> Self {
        let width = board_lines[0].len() as u16;
        let height = board_lines.len() as u16;
        let mut data = Vec::new();
        for line in board_lines {
            let mut row: Vec<Option<u32>> = Vec::new();
            for c in line.chars() {
                match c {
                    // Translate the chars to numbers.
                    'A'..='z' => {
                        if c == 'E' {
                            //'E' should be higher than 'z' so we need to special case this.
                            row.push(Some(36));
                        } else if c == 'S' {
                            row.push(Some(10));
                        } else {
                            row.push(c.to_digit(36));
                        }
                    }
                    _ => {
                        panic!("Couldn't parse input data into board.");
                    }
                }
            }
            data.push(row);
        }
        Self {
            width,
            height,
            data,
        }
    }

    fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if (dx + dy).abs() != 1 {
                    continue;
                }

                let new_position = Pos(position.0 + dx, position.1 + dy);
                if new_position.0 < 0
                    || new_position.0
                        >= self.width.try_into().expect("Overflow: grid width to i16!")
                    || new_position.1 < 0
                    || new_position.1
                        >= self
                            .height
                            .try_into()
                            .expect("Overflow: grid height to i16!")
                {
                    continue;
                }
                let board_value = self.data[new_position.1 as usize][new_position.0 as usize];
                if let Some(board_value) = board_value {
                    // compared to the version from https://github.com/gregstoll/rust-pathfinding/blob/main/src/lib.rs,
                    // we have to make sure steps with "height difference" > 1 are not taken. Here, we just exclude
                    // them from the list of valid successors. Alternatively, we could also just bump cost for these
                    // steps sufficiently.
                    if let Some(board_value_current) =
                        self.data[position.1 as usize][position.0 as usize]
                    {
                        if board_value as i16 - board_value_current as i16 <= 1 {
                            successors.push(Successor {
                                pos: new_position,
                                cost: board_value,
                            })
                        }
                    }
                }
            }
        }
        successors
    }
}

/// Helper function to get start ('S') position from input data.
fn get_start_coords_part1(input_data: &[&str]) -> (i16, i16) {
    let mut px_start: usize = 0;
    let mut py_start: usize = 0;
    for (py_idx, input_line) in input_data.iter().enumerate() {
        if let Some(start) = input_line.find('S') {
            px_start = start;
            py_start = py_idx;
        }
    }
    (px_start as i16, py_start as i16)
}

/// Helper function to get start ('S') position from input data.
fn get_start_coords_part2(input_data: &[&str]) -> Vec<Pos> {
    let mut px_start: usize;
    let mut py_start: usize;
    let mut start_coords: Vec<Pos> = Vec::new();
    for (py_idx, input_line) in input_data.iter().enumerate() {
        if let Some(start) = input_line.find('a') {
            px_start = start;
            py_start = py_idx;
            start_coords.push(Pos(px_start as i16, py_start as i16));
        }
    }
    start_coords
}

/// Helper function to get goal/end ('E') position from input data.
fn get_goal_coords(input_data: &[&str]) -> (i16, i16) {
    let mut px_goal: usize = 0;
    let mut py_goal: usize = 0;
    for (py_idx, input_line) in input_data.iter().enumerate() {
        if let Some(goal) = input_line.find('E') {
            px_goal = goal;
            py_goal = py_idx;
        }
    }
    (px_goal as i16, py_goal as i16)
}

fn main() {
    let input_data = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let steps_part1 = solution_part1(input_data.clone());
    println!("The goal was reached after {steps_part1} steps.");

    let steps_min = solution_part2(input_data);
    println!("Using the shortest way, the goal was reached after {steps_min} steps.");
}

fn solution_part2(input_data: Vec<&str>) -> usize {
    let start_coords = get_start_coords_part2(&input_data);
    let (goal_x, goal_y) = get_goal_coords(&input_data);
    let goal_pos = Pos(goal_x, goal_y);
    let board = Board::new(input_data);
    let mut steps: Vec<usize> = Vec::new();

    for start_pos in start_coords {
        let result = dijkstra(
            &start_pos,
            |p| {
                board
                    .get_successors(p)
                    .iter()
                    .map(|s| (s.pos, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| *p == goal_pos,
        );
        let result = result.expect("No path found.");
        steps.push(result.0.len() - 1);
    }
    let steps_min = steps
        .iter()
        .min()
        .expect("Way with minimum number of steps couldn't be calculated.");
    *steps_min
}

fn solution_part1(input_data: Vec<&str>) -> usize {
    let (start_x, start_y) = get_start_coords_part1(&input_data);
    let (goal_x, goal_y) = get_goal_coords(&input_data);

    let start_pos = Pos(start_x, start_y);
    let goal_pos = Pos(goal_x, goal_y);
    let board = Board::new(input_data);

    let result = dijkstra(
        &start_pos,
        |p| {
            board
                .get_successors(p)
                .iter()
                .map(|s| (s.pos, s.cost))
                .collect::<Vec<_>>()
        },
        |p| *p == goal_pos,
    );
    let result = result.expect("No path found.");
    result.0.len() - 1 // minus 1 since we have one less step than positions on board.
}

/// Do we have the correct number of steps for the test input?
#[test]
fn find_path_on_test_input_part1() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let steps = solution_part1(input_data);
    assert_eq!(steps, 31) // minus 1 since we have one less step than positions on board.
}

#[test]
fn find_path_on_test_input_part2() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let steps_min = solution_part2(input_data);
    assert_eq!(steps_min, 29)
}