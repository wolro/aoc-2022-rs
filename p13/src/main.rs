/// Advent of Code day 13
/// https://adventofcode.com/2022/day/13
use anyhow::Result;
use serde_json::Value;
use std::cmp::{max, Ordering};

fn main() -> Result<()> {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let l_packet_str = input_data[3];
    let r_packet_str = input_data[4];

    let l_packet: Value = serde_json::from_str(l_packet_str)?;
    let r_packet: Value = serde_json::from_str(r_packet_str)?;

    let l_vs_r: Ordering = comp_packets(&l_packet, &r_packet);

    Ok(())
}

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
            comp_packets(l_packet, &Value::Array(vec![r_packet.clone()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            comp_packets(&Value::Array(vec![l_packet.clone()]), r_packet)
        }
        _ => unreachable!(),
    }
}
