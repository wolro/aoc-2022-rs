use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// We have overlapping cleaning assignments, and we have to find who has it worst.
fn main() -> Result<()> {
    let file_path = "./input.txt";
    let assignments = read_input_data(file_path)?;

    // Riddle part 1: Full overlap between assignments?
    find_full_double_assignments(&assignments)?;
    // Riddle part 2: Partial overlap between assignments?
    find_partial_double_assignments(&assignments)?;
    Ok(())
}

/// First iteration, this is fore sure some ugly code and to the best we can do.
fn find_full_double_assignments(assignments: &Vec<String>) -> Result<usize> {
    let mut full_overlaps = 0;
    for assignment in assignments {
        let (elf_1, elf_2) = assignment.split_once(',').unwrap();
        let (start_1_str, end_1_str) = elf_1.split_once('-').unwrap();
        let (start_2_str, end_2_str) = elf_2.split_once('-').unwrap();
        let start_1 = start_1_str.parse::<usize>().unwrap();
        let start_2 = start_2_str.parse::<usize>().unwrap();
        let end_1 = end_1_str.parse::<usize>().unwrap();
        let end_2 = end_2_str.parse::<usize>().unwrap();
        if ((start_1 <= start_2) && (end_1 >= end_2)) | ((start_2 <= start_1) && (end_2 >= end_1)) {
            full_overlaps += 1;
        }
    }
    println!("{} elf teams have to clean the same areas.", &full_overlaps);
    Ok(full_overlaps)
}

/// First iteration, this is fore sure some ugly code and to the best we can do.
fn find_partial_double_assignments(assignments: &Vec<String>) -> Result<usize> {
    let mut overlaps = 0;
    for assignment in assignments {
        let (elf_1, elf_2) = assignment.split_once(',').unwrap();
        let (start_1_str, end_1_str) = elf_1.split_once('-').unwrap();
        let (start_2_str, end_2_str) = elf_2.split_once('-').unwrap();
        let start_1 = start_1_str.parse::<usize>().unwrap();
        let start_2 = start_2_str.parse::<usize>().unwrap();
        let end_1 = end_1_str.parse::<usize>().unwrap();
        let end_2 = end_2_str.parse::<usize>().unwrap();
        if ((start_1 <= end_2) && (end_1 >= start_2)) | ((start_2 <= end_1) && (end_2 >= start_1)) {
            overlaps += 1;
        }
    }
    println!("{} elf teams have to clean the same areas.", &overlaps);
    Ok(overlaps)
}

/// Read input data into vector of lines, each line being a string.
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let fh = File::open(file_path)?;
    let lines = BufReader::new(fh).lines();
    let match_results: Vec<String> = lines
        .collect::<Result<_, _>>()
        .expect("Something wrong with input data?");
    Ok(match_results)
}

/// Uses the example from the puzzle description to check if "find_full_double_assignment" works.
#[test]
fn test_find_full_double_assignment() {
    let test_assignments = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];
    let assignments = test_assignments.into_iter().map(String::from).collect();
    assert_eq!(find_full_double_assignments(&assignments).unwrap(), 2_usize)
}

/// Uses the example from the puzzle description to check if "find_partial_double_assignment" works.
#[test]
fn test_find_partial_double_assignment() {
    let test_assignments = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];
    let assignments = test_assignments.into_iter().map(String::from).collect();
    assert_eq!(
        find_partial_double_assignments(&assignments).unwrap(),
        4_usize
    )
}
