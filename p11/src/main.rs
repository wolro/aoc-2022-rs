use std::{thread::current, collections::VecDeque, iter::Enumerate};

/// Advent of Code day 10
/// https://adventofcode.com/2022/day/10
///
/// Monkeying around with a hand-written parser, again. At some point I will look into this "nom" crate. But not today.

#[derive(Debug, Clone, Default)]
struct Monkey {
    items: VecDeque<u64>,
    div: u64,
    op_op: char,
    fac: Option<u64>,
    receiver_true: u64,
    receiver_false: u64,
    insp_items: u64,
}
impl Monkey {
    fn insp_op(item: &u64, op: &char, fac: &Option<u64>) -> u64 {
        let mut operand: u64;
        if let Some(op) = fac {
            operand = *op;
        } else {
            operand = *item;
        }

        match op {
            '+' => (*item + operand)/3,
            '*' => (*item * operand)/3,
            _ => {
                println!("Can't update item value, error with item or Monkey?");
                *item
            }
        }
    }

    fn receiver(&self, item: &u64) -> usize {
        if *item % self.div == 0 {
            self.receiver_true.try_into().unwrap()
        } else {
            self.receiver_false.try_into().unwrap()
        }
    }
}

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
                            .collect::<VecDeque<u64>>();
                    } else if let Ok(sole_item) = split_line[1].parse::<u64>() {
                        current_monkey.items.push_back(sole_item);
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

// helper function for "monkey_parser" to extract struct field values from input data
fn parse_splitline(split_line: &[String], field: &str) -> u64 {
    let err_str = format!("Error parsing {field}.");
    split_line[1]
        .split(' ')
        .last()
        .expect(&err_str)
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let mut monkeys = monkey_parser(input_data);
    for _round in 0..1 {
        for mut monkey in &mut monkeys {
            monkey.items = monkey.items.iter()
                .map(|e| {
                    monkey.insp_items += 1;
                    Monkey::insp_op(e, &monkey.op_op, &monkey.fac) })
                .collect();
        }
        let nr_monkeys = monkeys.len();
        for m_idx in 0..nr_monkeys {
            for(i_idx, item) in monkeys[m_idx].clone().items.iter().enumerate() {
                let rec_idx = monkeys[m_idx].receiver(&monkeys[m_idx].items[i_idx]);
                monkeys[rec_idx].items.push_back(*item);
            }
            monkeys[m_idx].items.clear();
        }


    }
    dbg!(&monkeys);
    let mut inspections = monkeys.iter()
    .map(|e| e.insp_items )
    .collect::<Vec<_>>();

    inspections.sort();
    let inspections = inspections.iter().rev().collect::<Vec<_>>();
    let score = inspections[0] * inspections[1];
    dbg!(score);
}

