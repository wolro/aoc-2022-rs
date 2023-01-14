/// Advent of Code day 1
/// https://adventofcode.com/2022/day/1
use anyhow::Result; // "anyhow" crate wraps arbitrary error types so we don't have to.
use std::fs;

fn main() -> Result<()> {
    let file_path = "./input.txt";

    let hp_list = read_input_data(file_path)?;
    let total_hp_per_elf = total_hp_per_elf(hp_list)?;

    // find elf with maximum "hp" or "food points"
    // (solution to part 1)
    let max_hp = total_hp_per_elf
        .iter()
        .max()
        .expect("Couldn't find maximum, something wrong with input vector?");
    println!("Elf with most food carries equivalent of {} hp.", &max_hp);

    // for part 2, we have to sum up the food carried by the three
    // most-loaded elves. So we sort, reverse, and add up the first three entries.
    let mut total_hp_sorted = total_hp_per_elf.clone();
    total_hp_sorted.sort();
    total_hp_sorted.reverse();
    let max_three_hp: u32 = total_hp_sorted[..3].iter().sum();
    println!(
        "Three elves with most food carry a total equivalent of {} hp.",
        &max_three_hp
    );

    Ok(())
}

/// Parse Vector with hp strings into int, split at "none" elements into
/// a aggregate resulting arrays into sums to return the total hp represented
/// by the food carried by each elf.
fn total_hp_per_elf(hp_list: Vec<String>) -> Result<Vec<u32>> {
    let hp_list_int = hp_list
        .iter()
        .map(|v| v.parse::<u32>().ok()) // casts into Option<u32>, empty strings result into NOne
        .collect::<Vec<_>>();
    let total_hp_per_elf = hp_list_int
        .split(|line| line.is_none()) // now we can split at the None elements and get a vector of arrays of Option <u32>
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u32>()) // map arrays into their sums
        .collect();
    Ok(total_hp_per_elf)
}

/// Just read everything into a string and split into a vector afterwards.
/// Note that I started out using the "csv" crate, but this automatically
/// removes empty lines, which act as delimiter between each elf's inventory
/// here.
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let hp_string = fs::read_to_string(file_path).expect("Reading file didn't work, wrong path?");
    let hp_list: Vec<String> = hp_string.split('\n').map(String::from).collect();
    Ok(hp_list)
}
