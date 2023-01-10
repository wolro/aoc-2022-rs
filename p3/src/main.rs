use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// We have to find misplaced items and group ID badges in the elves' backpacks.
/// See https://adventofcode.com/2022/day/3.
fn main() -> Result<()> {
    let file_path = "./input.txt";
    let rucksacks = read_input_data(file_path)?;

    // first, the misplaced items
    let misplaced_items = find_misplaced_items(&rucksacks)?;
    let alphabet = get_alphabet();

    let priority_sum = get_priority_sum(misplaced_items, &alphabet)?;
    println!("The sum of misplaced item priorities is: {}", priority_sum);

    // now the badges
    let badges = find_badges(&rucksacks)?;
    let mut badge_priorities = Vec::new();
    for item in badges {
        badge_priorities.push(get_priority_score(&item, &alphabet)?);
    }

    let badge_priority_sum: u32 = badge_priorities.iter().sum();
    println!("The sum of badge priorities is: {}", badge_priority_sum);

    Ok(())
}

/// Read input data into vector of lines, each line being a string
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let fh = File::open(file_path)?;
    let lines = BufReader::new(fh).lines();
    let match_results: Vec<String> = lines
        .collect::<Result<_, _>>()
        .expect("Something wrong with input data?");
    Ok(match_results)
}

/// get our alphabet in an order where the index of a char corresponds
/// to its priority as described here: https://adventofcode.com/2022/day/3
fn get_alphabet() -> String {
    String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap()
}

/// Split rucksack into two equally sized compartments. We can
/// rely on the fact that this works since all rucksacks contain
/// an even amount of items.
fn bisect_rucksack(rucksack: &str) -> Result<(&str, &str)> {
    Ok(rucksack.split_at(rucksack.len() / 2))
}

/// Find any "items" present in both rucksack compartments.
fn find_misplaced_items(rucksacks: &Vec<String>) -> Result<Vec<String>> {
    let mut misplaced_items: Vec<String> = Vec::new();
    for rucksack in rucksacks {
        let (comp1, comp2) = bisect_rucksack(&rucksack)?;
        let common_item = comp1
            .chars()
            .find(|c| comp2.contains(*c))
            .expect("No common items present?");
        misplaced_items.push(common_item.to_string());
    }
    Ok(misplaced_items)
}

/// Find "badges", i.e. common items within groups of three consecutive
/// elf backpacks.
fn find_badges(rucksacks: &Vec<String>) -> Result<Vec<String>> {
    let backpacks = rucksacks.to_owned();
    let groups: Vec<&[String]> = backpacks.chunks(3).collect();
    let mut badges: Vec<String> = Vec::new();
    for group in groups {
        let badge: char = group[0]
            .chars()
            .find(|c| group[1].contains(*c) && group[2].contains(*c))
            .expect("No badge found?");
        badges.push(badge.to_string());
    }
    Ok(badges)
}

/// Calculate sum of item priorites, making use of "get_priority_score".
fn get_priority_sum(misplaced_items: Vec<String>, alphabet: &str) -> Result<u32, anyhow::Error> {
    let mut priorities = Vec::new();
    for item in misplaced_items {
        priorities.push(get_priority_score(&item, alphabet)?);
    }
    let priority_sum: u32 = priorities.iter().sum();
    Ok(priority_sum)
}

/// Use index of char representing an item in the constructed "alphabet"
/// as priority score.
fn get_priority_score(item: &str, alphabet: &str) -> Result<u32> {
    let item_idx = alphabet
        .chars()
        .position(|e| e == item.chars().next().expect("Item is not a valid &str?"))
        .expect("Item has no representation in alphabet?");
    let priority_score = item_idx as u32 + 1;
    Ok(priority_score)
}

/// "bisect_rucksack" is not validated to work for odd numbers so let's see
/// whether we can safely assume an even number of items per backpack, as understood
/// from the puzzle description.
#[test]
fn test_even_number_of_items_in_rucksacks() {
    let file_path = "./input.txt";
    let rucksacks = read_input_data(file_path).unwrap();
    for rucksack in rucksacks {
        assert_eq!(rucksack.len(), 2 * rucksack.len() / 2); // this should just panic for odd rucksack lengthts
    }
}

/// Check if we can split a string, and recover the original
/// by putting it back together again.
#[test]
fn test_bisect_rucksack() {
    let test_rucksack = "owWilPIkdloPowkeIDKk";
    let (comp1, comp2) = bisect_rucksack(test_rucksack).unwrap();
    let rebuilt_rucksack: String = String::from(comp1) + comp2;
    assert_eq!(test_rucksack, &rebuilt_rucksack);
}
