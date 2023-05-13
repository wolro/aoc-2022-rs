/// Advent of Code day 13
/// https://adventofcode.com/2022/day/13
///
/// This was fun. One thing that was spoiled for me by previously clicking on an article
/// about this riddle is that "serde_json" can be used to directly parse the input.
/// I also needed some hints from other people's solutions to get the recursion for
/// comparison right. Part 2 was straightforward.
///
/// For this one, it will be interesting to review other solutions. I know I can implement
/// "Ord" for the packets, so this would be interesting to try. I am also not sure if
/// there is not a more elegant way than sticking all entries into a sorting algorithm.
use anyhow::Result;
use serde_json::Value;
use std::cmp::{max, Ordering};

fn main() -> Result<()> {
    let input_data = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let nr_ordered = part1_solution(input_data.clone())?;
    println!("Number of correctly ordered packets: {}", nr_ordered);

    let decoder_key = part2_solution(input_data)?;
    println!("Decoder key: {}", decoder_key);

    Ok(())
}

fn bubbles(packet_vec: &mut Vec<Value>) {
    let mut swapped;

    loop {
        swapped = false;

        for packet_idx in 0..packet_vec.len() - 1 {
            if comp_packets(&packet_vec[packet_idx], &packet_vec[packet_idx + 1])
                == Ordering::Greater
            {
                packet_vec.swap(packet_idx, packet_idx + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

/// Comparison function used for both parts.
fn comp_packets(l_packet: &Value, r_packet: &Value) -> Ordering {
    match (l_packet, r_packet) {
        (Value::Number(x), Value::Number(y)) => x.as_i64().unwrap().cmp(&y.as_i64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..max(a.len(), b.len()) {
                match (a.get(i), b.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match comp_packets(x, y) {
                        Ordering::Equal => {}
                        c => return c,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => {
            // Stick the "bare" number into an array so that after a sufficient
            // number of recursion we end up with two numbers or vectors.
            comp_packets(l_packet, &Value::Array(vec![r_packet.clone()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            comp_packets(&Value::Array(vec![l_packet.clone()]), r_packet)
        }
        _ => unreachable!(),
    }
}

/// Find dividers by only considering numbers and arrays with single entries,
/// checking recursively for the divider numbers if this is the case, and
/// toss everything else.
fn check_for_dividers(packet: &Value) -> bool {
    match packet {
        Value::Number(x) => {
            matches!(x.as_i64().unwrap(), 2 | 6)
        }
        Value::Array(a) => match a.len() {
            1 => check_for_dividers(a.get(0).unwrap()),
            _ => false,
        },
        _ => unreachable!(),
    }
}

fn part1_solution(input_data: Vec<&str>) -> Result<usize> {
    let mut ordered_idcs: Vec<usize> = Vec::new();
    let mut pair_idx = 1;

    for packet_idx in (0..input_data.len()).step_by(3) {
        let l_packet = serde_json::from_str(input_data[packet_idx])?;
        let r_packet = serde_json::from_str(input_data[packet_idx + 1])?;

        if comp_packets(&l_packet, &r_packet) == Ordering::Less {
            ordered_idcs.push(pair_idx);
        }
        pair_idx += 1;
    }

    let nr_ordered: usize = ordered_idcs.iter().sum();
    Ok(nr_ordered)
}

fn part2_solution(mut input_data: Vec<&str>) -> Result<usize, anyhow::Error> {
    input_data.retain(|&e| !e.is_empty());
    input_data.push("[[2]]");
    input_data.push("[[6]]");
    let mut packet_vec: Vec<Value> = Vec::new();
    for item in input_data {
        packet_vec.push(serde_json::from_str(item)?);
    }
    bubbles(&mut packet_vec);
    let mut divider_idcs: Vec<usize> = Vec::new();
    for (idx, packet) in packet_vec.iter().enumerate() {
        if check_for_dividers(packet) {
            divider_idcs.push(idx + 1);
        }
    }
    let decoder_key: usize = divider_idcs.iter().product();
    Ok(decoder_key)
}

#[test]
fn part1_validate_on_testdata() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let nr_ordered = part1_solution(input_data).unwrap();
    assert_eq!(nr_ordered, 13);
}

#[test]
fn part2_validate_on_testdata() {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let result_p2 = part2_solution(input_data).unwrap();
    assert_eq!(result_p2, 140);
}
