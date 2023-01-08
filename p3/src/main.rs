use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let file_path = "./input.txt";
    let rucksacks = read_input_data(file_path)?;

    let misplaced_items = find_misplaced_items(rucksacks)?;
    let alphabet = get_alphabet();

    let mut priorities = Vec::new();
    for item in misplaced_items {
        priorities.push(get_priority_score(&item, &alphabet)?);
    }

    let priority_sum: u32 = priorities.iter().sum();
    println!("The sum of misplaced item priorities is: {}", priority_sum);

    Ok(())
}

/// Find any "items" present in both rucksack compartments.
fn find_misplaced_items(rucksacks: Vec<String>) -> Result<Vec<String>, anyhow::Error> {
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

/// Read input data into vector of lines, each line being a string
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let fh = File::open(file_path)?;
    let lines = BufReader::new(fh).lines();
    let match_results: Vec<String> = lines
        .collect::<Result<_, _>>()
        .expect("Something wrong with input data?");
    Ok(match_results)
}

/// Split rucksack into two equally sized compartments. We can
/// rely on the fact that this works since all rucksacks contain
/// an even amount of items.
fn bisect_rucksack(rucksack: &str) -> Result<(&str, &str)> {
    Ok(rucksack.split_at(rucksack.len() / 2))
}

/// get our alphabet in an order where the index of a char corresponds
/// to its priority as described here: https://adventofcode.com/2022/day/3
fn get_alphabet() -> String {
    String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap()
}

/// Use index of char representing an item in the constructed "alphabet"
/// as priority score
fn get_priority_score(item: &str, alphabet: &str) -> Result<u32> {
    let item_idx = alphabet
        .chars()
        .position(|e| e == item.chars().next().expect("Item is not a valid &str?"))
        .expect("Item has no representation in alphabet?");
    let priority_score = item_idx as u32 + 1;
    Ok(priority_score)
}

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
