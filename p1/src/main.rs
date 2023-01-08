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


// Split vector into each elf's inventory by using empty lines as delimiters,
// parse strings into numbers and return sum for each elf as a vector.
fn total_hp_per_elf(hp_list: Vec<String>) -> Result<Vec<u32>> {
    let hp_per_elf: Vec<_> = hp_list.split(|item| item.is_empty()).collect();
    // "Vec<_>" aboe means: Compiler, this is a vector, but please figure out the inner type yourself.
    let mut total_hp_per_elf: Vec<u32> = Vec::new();
    for item in hp_per_elf {
        let hps: Vec<_> = item.iter().map(|e| e.parse::<u32>().unwrap()).collect();
        let hpsum: u32 = hps.iter().sum();
        total_hp_per_elf.push(hpsum);
    }
    Ok(total_hp_per_elf)
}

// Just read everything into a string and split into a vector afterwards.
// Note that I started out using the "csv" crate, but this automatically
// removes empty lines, which act as delimiter between each elf's inventory
// here.
fn read_input_data(file_path: &str) -> Result<Vec<String>> {
    let hp_string = fs::read_to_string(file_path).expect("Reading file didn't work, wrong path?");
    let hp_list: Vec<String> = hp_string.split('\n').map(String::from).collect();
    Ok(hp_list)
}
