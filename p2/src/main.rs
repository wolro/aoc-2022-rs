/// Advent of Code day 2
/// https://adventofcode.com/2022/day/2
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file_path = "./input.txt";
    let match_results = read_input_data(file_path)?;

    // Using iterators and maps to assign a function element-wise
    // to a collection is actually quite nice.
    let scores_part1: Vec<u32> = match_results
        .iter()
        .map(|e| evaluate_round_part1(e).expect("Something wrong with input data?"))
        .collect();

    let total_score_part1: u32 = scores_part1.iter().sum();
    println!(
        "The total score based on our initial assumtion is: {}",
        total_score_part1
    );

    // Since we didn't guess correctly what the elf meant, we have to redo
    // the same thing with the assignment defined in "evaulate_round_part2".
    let scores_part2: Vec<u32> = match_results
        .iter()
        .map(|e| evaluate_round_part2(e).expect("Something wrong with input data?"))
        .collect();

    let total_score_part2: u32 = scores_part2.iter().sum();
    println!(
        "The total score based the elf's description is: {}",
        total_score_part2
    );

    Ok(())
}

/// Read input data into vector of lines, each line being a string
/// looking like e.g. "A X".
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let fh = File::open(file_path)?;
    let lines = BufReader::new(fh).lines();
    let match_results: Vec<String> = lines
        .collect::<Result<_, _>>()
        .expect("Something wrong with input data?");
    Ok(match_results)
}

/// Assign score for a given line according to description of part 1
/// of the riddle as described here: https://adventofcode.com/2022/day/2
fn evaluate_round_part1(match_result: &str) -> Result<u32> {
    let (ene, me) = match_result
        .split_once(' ')
        .unwrap_or_else(|| panic!("Something wrong with input data: {} ?", match_result));

    let score: u32;

    // Being able to match on tuples is nifty.
    match (ene, me) {
        ("A", "X") => score = 4,
        ("B", "X") => score = 1,
        ("C", "X") => score = 7,
        ("A", "Y") => score = 8,
        ("B", "Y") => score = 5,
        ("C", "Y") => score = 2,
        ("A", "Z") => score = 3,
        ("B", "Z") => score = 9,
        ("C", "Z") => score = 6,
        _ => score = 0,
    };

    if score == 0 {
        println!("Wrong input: {}, {}", ene, me);
    }

    Ok(score)
}

/// Assign score for each line as described in part 2 of the riddle.
/// https://adventofcode.com/2022/day/2
fn evaluate_round_part2(match_result: &str) -> Result<u32> {
    let (ene, me) = match_result
        .split_once(' ')
        .unwrap_or_else(|| panic!("Something wrong with input data: {} ?", match_result));

    let score: u32;

    match (ene, me) {
        ("A", "X") => score = 3,
        ("B", "X") => score = 1,
        ("C", "X") => score = 2,
        ("A", "Y") => score = 4,
        ("B", "Y") => score = 5,
        ("C", "Y") => score = 6,
        ("A", "Z") => score = 8,
        ("B", "Z") => score = 9,
        ("C", "Z") => score = 7,
        _ => score = 0,
    };

    if score == 0 {
        println!("Wrong input: {}, {}", ene, me);
    }

    Ok(score)
}

/// Make sure matches involving paper and rock produce the correct score (part 1).
#[test]
fn test_paper_rock_part1() {
    let test_res1 = "A Y";
    let test_res2 = "B X";

    assert_eq!(8, evaluate_round_part1(test_res1).unwrap());
    assert_eq!(1, evaluate_round_part1(test_res2).unwrap());
}

/// Make sure matches involving rock and scissors produce the correct score (part 1).
#[test]
fn test_rock_scissors_part1() {
    let test_res1 = "A Z";
    let test_res2 = "C X";

    assert_eq!(3, evaluate_round_part1(test_res1).unwrap());
    assert_eq!(7, evaluate_round_part1(test_res2).unwrap());
}

/// Make sure matches involving scissors and paper produce the correct score (part 1).
#[test]
fn test_scissors_paper_part1() {
    let test_res1 = "B Z";
    let test_res2 = "C Y";

    assert_eq!(9, evaluate_round_part1(test_res1).unwrap());
    assert_eq!(2, evaluate_round_part1(test_res2).unwrap());
}

/// Make sure draw matches produce the correct score (part 1).
#[test]
fn test_draw_part1() {
    let test_res1 = "A X";
    let test_res2 = "B Y";
    let test_res3 = "C Z";

    assert_eq!(4, evaluate_round_part1(test_res1).unwrap());
    assert_eq!(5, evaluate_round_part1(test_res2).unwrap());
    assert_eq!(6, evaluate_round_part1(test_res3).unwrap());
}
