/// Advent of Code day 5
/// https://adventofcode.com/2022/day/5
///
/// This was actually interesting, and getting the input into a usable form
/// required a bit of playing around.
///
/// I went with representing all the data in terms of vectors. Probably
/// noobish, on the other hand, we have a lot of dynamic  shuffling of items
/// between containers, so vectors are perhaps OK?
/// We'll see after looking at other peoples' solutions...
use anyhow::Result;

fn main() -> Result<()> {
    // Read data; just learned that we can just inline the puzzle input, so why not?
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();

    // Split data into relevant parts
    let (crate_stack_raw, instructions, baseline) = parse_and_segment_input(lines)?;

    // perform algorithm steps for part 1 and get result
    let crate_stack_copy = crate_stack_raw.clone(); // for part 2
    let mut crates = build_crate_vec(baseline, crate_stack_raw)?;
    for instruction in &instructions {
        move_crates_by_instruction_part1(instruction, &mut crates);
    }
    let result = get_crates_on_top(crates)?;
    println!(
        "The uppermost crates on each stack are (part 1): {}",
        result
    );

    // perform algorithm steps for part 2 and get result
    let mut crates2 = build_crate_vec(baseline, crate_stack_copy)?;
    for instruction in &instructions {
        move_crates_by_instruction_part2(instruction, &mut crates2);
    }
    let result2 = get_crates_on_top(crates2)?;
    println!(
        "The uppermost crates on each stack are (part 2): {}",
        result2
    );

    Ok(())
}

/// Parse input data into:
/// a) the representation of our crate stack as a vector of &str,
/// b) the instruction list as a vector of &str,
/// c) the "base line" enumerating the crate stacks in corresponding with the instruction list.
fn parse_and_segment_input(lines: Vec<&str>) -> Result<(Vec<&str>, Vec<&str>, &str)> {
    let input_sections = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let mut crate_stack = input_sections[0].to_owned();
    let instructions = input_sections[1].to_owned();
    let baseline = crate_stack.pop().unwrap();
    Ok((crate_stack, instructions, baseline))
}

/// The input data has a cute vertical representation of the crate stacks. This function
/// parses this representation into a vector of vectors of strings. Strings represent
/// crates, inner vectors each single stacks, and the outer vector collects the stacks.
fn build_crate_vec(baseline: &str, crate_stack: Vec<&str>) -> Result<Vec<Vec<String>>> {
    // get numbers identifying stacks
    let stack_nrs: Vec<_> = baseline
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // get indices of the chars representing the boxes
    let mut stack_idcs = Vec::new();
    for str_idx in 1..(stack_nrs.last().unwrap() + 1) {
        stack_idcs.push(
            baseline
                .chars()
                .position(|ch| {
                    ch == char::from_digit(str_idx, 10).expect("Couldn't get stack indices.")
                })
                .expect("Invalid string entry."),
        );
    }
    // build crate vector representation.
    let mut crate_stack_vecs = Vec::new();
    for stack_idx in stack_idcs {
        let mut tmp_vec = Vec::new();
        // Build vector for each stack. In the original representation, the "upper" crates to be manupulated
        // are the first ones. Here, we revert the vector so we can rather use "pop" and "push" on the vector
        // tails.
        for line in crate_stack.iter().rev() {
            tmp_vec.push(
                line.chars()
                    .nth(stack_idx)
                    .expect("Couldn't get crate.")
                    .to_string(),
            );
        }

        crate_stack_vecs.push(tmp_vec.clone());
    }
    // "crate_stack_vecs" contains empty strings where there were no boxes. let's get rid of them.
    let mut crates = Vec::new();
    for mut stack_vec in crate_stack_vecs {
        stack_vec.retain(|x| x != " ");
        crates.push(stack_vec);
    }
    Ok(crates)
}

/// This function takes a mutable reference to "crates" and shuffles around the crates
/// according to the puzzle instructions, part 1 ("CrateMover 9000" :-)).
fn move_crates_by_instruction_part1(step1: &str, crates: &mut [Vec<String>]) {
    // get the numbers telling us what we should do from the instruction string.
    let numbers: Vec<_> = step1
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .filter_map(|e| e.ok())
        .collect();
    // translate to indices, leave quantity of moved boxes alone
    let (source, target, quantity) = (numbers[1] - 1, numbers[2] - 1, numbers[0]);
    // Shuffling has to be done sequentially in a loop as per instructions, otherwise the order
    // of the boxes will be wrong.
    for _step in 0..quantity {
        let moved_crate = crates[source]
            .pop()
            .expect("Couldn't remove crate from stack.");
        crates[target].push(moved_crate);
    }
}

/// This function takes a mutable reference to "crates" and shuffles around the crates
/// according to the puzzle instructions, part 2 ("CrateMover 9001" :-)). Actually
/// this was my first solutions since I didn't read the instructions of part 1 properly...
fn move_crates_by_instruction_part2(step1: &str, crates: &mut [Vec<String>]) {
    let segs: Vec<_> = step1
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .filter_map(|e| e.ok())
        .collect();
    let (source, target, quantity) = (segs[1] - 1, segs[2] - 1, segs[0]);
    let removed_crates = crates[source].len().saturating_sub(quantity);
    let mut stack_tail: Vec<String> = crates[source].drain(removed_crates..).collect();
    crates[target].append(&mut stack_tail);
}

/// Collect the crates "on top" (last elements of each stack vector) into a string to be
/// typed into the puzzle solution form.
fn get_crates_on_top(crates: Vec<Vec<String>>) -> Result<String> {
    let mut result_str = String::new();
    for cr in crates {
        result_str.push_str(cr.last().unwrap().as_ref());
    }
    Ok(result_str)
}

/// Apply our algorithm to test data from the puzzle description, part 1
#[test]
fn test_algo_part1_on_test_input() {
    // input data from example
    let lines = include_str!("../input_test.txt") // Just learned that we can just inline the puzzle input, so why not?
        .lines()
        .collect::<Vec<_>>();
    // apply algorithm steps
    let (crate_stack_raw, instructions, baseline) = parse_and_segment_input(lines).unwrap();
    let mut crates = build_crate_vec(baseline, crate_stack_raw).unwrap();
    for instruction in instructions {
        move_crates_by_instruction_part1(instruction, &mut crates);
    }
    let result = get_crates_on_top(crates).unwrap();

    assert_eq!(result, "CMZ");
}

/// Apply our algorithm to test data from the puzzle description, part 2
#[test]
fn test_algo_part2_on_test_input() {
    // input data from example
    let lines = include_str!("../input_test.txt") // Just learned that we can just inline the puzzle input, so why not?
        .lines()
        .collect::<Vec<_>>();
    // apply algorithm steps
    let (crate_stack_raw, instructions, baseline) = parse_and_segment_input(lines).unwrap();
    let mut crates = build_crate_vec(baseline, crate_stack_raw).unwrap();
    for instruction in instructions {
        move_crates_by_instruction_part2(instruction, &mut crates);
    }
    let result = get_crates_on_top(crates).unwrap();

    assert_eq!(result, "MCD");
}
