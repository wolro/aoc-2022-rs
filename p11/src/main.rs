use std::thread::current;

/// Advent of Code day 10
/// https://adventofcode.com/2022/day/10
///
/// Monkeying around with a hand-written parser, again. At some point I will look into this "nom" crate. But not today.

#[derive(Debug, Clone, Default)]
struct Monkey {
    items: Vec<u32>,
    div: u32,
    op_op: char,
    fac: Option<u32>,
    receiver_true: u32,
    receiver_false: u32,
}
impl Monkey {
    fn insp_op(item: u32, op: char, fac: Option<u32>) -> u32 {
        let mut operand: u32;
        if let Some(op) = fac {
            operand = op;
        } else {
            operand = item;
        }

        match op {
            '+' => item + operand,
            '*' => item * operand,
            _ => {
                println!("Can't update item value, error with item or Monkey?");
                dbg!(&item);
                dbg!(&operand);
                dbg!(&op);
                item
            }
        }
    }

    fn receiver(self, item: u32) -> u32 {
        if item % self.div == 0 {
            self.receiver_true
        } else {
            self.receiver_false
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
                            .map(|e| e.trim().parse::<u32>().expect("Error parsing items."))
                            .collect::<Vec<u32>>();
                    } else if let Ok(sole_item) = split_line[1].parse::<u32>() {
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
                    current_monkey.fac = operand.trim().parse::<u32>().ok()
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
fn parse_splitline(split_line: &[String], field: &str) -> u32 {
    let err_str = format!("Error parsing {field}.");
    split_line[1]
        .split(' ')
        .last()
        .expect(&err_str)
        .parse::<u32>()
        .unwrap()
}

fn main() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let monkeys = monkey_parser(input_data);
    dbg!(&monkeys);
}
