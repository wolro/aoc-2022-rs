/// Advent of Code day 11
/// https://adventofcode.com/2022/day/11
///
/// Monkeying around with a hand-written parser, again. At some point I will look into this "nom" crate. But not today.
///
/// Otherwise, needed to look at other people's solutions for hints on this.
/// First, issues with borrow checker with loops accessing both the monkey vector
/// and its elements. Resolved by looping over index and having only a mutable borrow
/// to a singular monkey out of the vector.
///
/// Second, I needed hints for part 2. Never heard of the "Chinese Remainder Theorem"
/// before. On the bright side, I made use of passing functions as arguments for the
/// first time in Rust.

/// The monkey struct.
#[derive(Debug, Clone, Default)]
struct Monkey {
    items: Vec<u64>,
    div: u64,            // Divisor, see input data
    op_op: char,         // Operator used in "Operation" (+ or *), see input data
    fac: Option<u64>, // Second operand for "Operation". If None, use "item" (worry value), see e.g. Monkey 2 in test input data.
    receiver_true: u64, // Index of monkey being thrown to if test condition is true
    receiver_false: u64, // Index of monkey being thrown to if test condition is false
    insp_items: u64,  // Number of times this monkey has inspected an item
}
impl Monkey {
    /// Model inspection operation. Takes function "f" to apply "worry reduction"
    /// (either division or remainder for part 1 and 2, respectively).
    fn insp_op(
        item: &u64,
        op: &char,
        fac: &Option<u64>,
        f: &dyn Fn(u64, u64) -> u64,
        worry_div: u64,
    ) -> u64 {
        let operand: u64;
        if let Some(op) = fac {
            operand = *op;
        } else {
            operand = *item;
        }

        match op {
            '+' => f(*item + operand, worry_div),
            '*' => f(*item * operand, worry_div),
            _ => {
                println!("Can't update item value, error with item or Monkey?");
                *item
            }
        }
    }

    /// Returns monkey an "item" is thrown to.
    fn receiver(&self, item: &u64) -> usize {
        if *item % self.div == 0 {
            self.receiver_true.try_into().unwrap()
        } else {
            self.receiver_false.try_into().unwrap()
        }
    }
}

/// Divide worry by 3 after inspection, see instructions for part 1.
fn worry_reducer_part1(worry: u64, reducer: u64) -> u64 {
    worry / reducer
}

/// Remainder operation to reduce ginormous "worry levels".
fn worry_reducer_part2(value: u64, reducer: u64) -> u64 {
    value % reducer
}

/// The great monkey parser. Didn't want to rely on fixed string splitting positions,
/// so I matched on the contents of the "prefix" (part before colon). Made the parser
/// more verbose, but so be it.
fn monkey_parser(input_data: Vec<&str>) -> Vec<Monkey> {
    // split input into segments corresponding to each monkey
    let input_monkeys: Vec<_> = input_data
        .split(|&e| e.is_empty())
        .filter(|v| !v.is_empty())
        .collect();

    // initialize default monkey and monkey list
    let mut monkeys: Vec<Monkey> = Vec::new();
    let null_monkey: Monkey = Default::default();
    let mut current_monkey;

    // parse each monkey data into corresponding struct and return vector with all of these monkeys
    for input_monkey in input_monkeys {
        // initialize new monkey
        current_monkey = null_monkey.clone();
        for line in input_monkey {
            let split_line = line
                .split(':')
                .map(|e| e.to_owned())
                .collect::<Vec<String>>();
            let prefix = &split_line[0].clone();
            match prefix {
                // parse item list
                prefix if prefix.contains("Starting") => {
                    if split_line[1].contains(',') {
                        current_monkey.items = split_line[1]
                            .split(',')
                            .map(|e| e.trim().parse::<u64>().expect("Error parsing items."))
                            .collect::<Vec<u64>>();
                    } else if let Ok(sole_item) = split_line[1].trim().parse::<u64>() {
                        current_monkey.items.push(sole_item);
                    }
                }
                // parse operation
                prefix if prefix.contains("Operation") => {
                    let eq_rhs = split_line[1]
                        .split('=')
                        .last()
                        .expect("Error parsing operation: cannot split equation?")
                        .to_owned();
                    match eq_rhs.clone() {
                        eq_rhs if eq_rhs.contains('*') => {
                            current_monkey.op_op = '*';
                        }
                        eq_rhs if eq_rhs.contains('+') => {
                            current_monkey.op_op = '+';
                        }
                        _ => {
                            println!("Error parsing operation: unknown operation?")
                        }
                    }
                    let operand = eq_rhs
                        .split(current_monkey.op_op)
                        .last()
                        .expect("Error parsing operation: cannot get operand?");
                    current_monkey.fac = operand.trim().parse::<u64>().ok()
                }
                // parse divisor
                prefix if prefix.contains("Test") => {
                    current_monkey.div = parse_splitline(&split_line, "div");
                }
                // parse target monkeys for each "Test" outcome
                prefix if prefix.contains("If true") => {
                    current_monkey.receiver_true = parse_splitline(&split_line, "receiver_true");
                }
                prefix if prefix.contains("If false") => {
                    current_monkey.receiver_false = parse_splitline(&split_line, "receiver_false");
                }
                _ => {}
            }
        }
        monkeys.push(current_monkey.clone());
    }
    monkeys
}

/// Helper function for "monkey_parser" to extract struct field values
/// for "div", "receiver_true" and "receiver_false" from input data.
fn parse_splitline(split_line: &[String], field: &str) -> u64 {
    let err_str = format!("Error parsing {field}.");
    split_line[1]
        .split(' ')
        .last()
        .expect(&err_str)
        .parse::<u64>()
        .unwrap()
}

/// Execute one round of inspection and throwing.
fn execute_round(monkeys: &mut Vec<Monkey>, reducer: &dyn Fn(u64, u64) -> u64, worry_div: u64) {
    for m_idx in 0..monkeys.len() {
        let monkey = &mut monkeys[m_idx];
        // inspect and update worry levels
        monkey.items = monkey
            .items
            .iter_mut()
            .map(|e| {
                monkey.insp_items += 1;
                Monkey::insp_op(e, &monkey.op_op, &monkey.fac, reducer, worry_div)
            })
            .collect();
        // throw items
        for (i_idx, item) in monkeys[m_idx].clone().items.iter().enumerate() {
            let rec_idx = monkeys[m_idx].receiver(&monkeys[m_idx].items[i_idx]);
            monkeys[rec_idx].items.push(*item);
        }
        monkeys[m_idx].items.clear();
    }
}

fn main() {
    solution_part1();
    solution_part2();
}

fn solution_part1() {
    let input_data = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let mut monkeys = monkey_parser(input_data);

    for _round in 0..20 {
        execute_round(&mut monkeys, &worry_reducer_part1, 3);
    }
    // get number of inspections performed by each monkey
    let mut inspections = monkeys.iter().map(|e| e.insp_items).collect::<Vec<_>>();

    inspections.sort(); // sort in ascending order
    let inspections = inspections.iter().rev().collect::<Vec<_>>(); // reverse order
    let score = inspections[0] * inspections[1]; // score, see puzzle description
    dbg!(score);
}

fn solution_part2() {
    let input_data = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let mut monkeys = monkey_parser(input_data);
    let div_prod = monkeys.iter().map(|e| e.div).product();
    // TBH i have no idea why the remainder operation using "div_prod"
    // leaves the division check unaffected. But I am not sure if I care sufficiently.

    for _round in 0..10000 {
        execute_round(&mut monkeys, &worry_reducer_part2, div_prod);
    }

    let mut inspections = monkeys.iter().map(|e| e.insp_items).collect::<Vec<_>>();

    inspections.sort();
    let inspections = inspections.iter().rev().collect::<Vec<_>>();
    let score = inspections[0] * inspections[1];
    dbg!(score);
}

#[test]
fn test_monkey_slinging_part1() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let mut monkeys = monkey_parser(input_data);

    for _round in 0..20 {
        execute_round(&mut monkeys, &worry_reducer_part1, 3);
    }

    let mut inspections = monkeys.iter().map(|e| e.insp_items).collect::<Vec<_>>();

    inspections.sort();
    let inspections = inspections.iter().rev().collect::<Vec<_>>();
    let score = inspections[0] * inspections[1];
    assert_eq!(score, 10605);
}

#[test]
fn test_monkey_slinging_part2() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let mut monkeys = monkey_parser(input_data);
    let div_prod = monkeys.iter().map(|e| e.div).product();

    for _round in 0..10000 {
        execute_round(&mut monkeys, &worry_reducer_part2, div_prod);
    }

    let mut inspections = monkeys.iter().map(|e| e.insp_items).collect::<Vec<_>>();

    inspections.sort();
    let inspections = inspections.iter().rev().collect::<Vec<_>>();
    let score = inspections[0] * inspections[1];
    assert_eq!(score, 2713310158);
}
