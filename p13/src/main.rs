/// Advent of Code day 13
/// https://adventofcode.com/2022/day/13
use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
    let input_data = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    // dbg!(input_data);

    let l_packet_str = input_data[7];
    let r_packet_str = input_data[4];

    let l_packet_json: Value = serde_json::from_str(l_packet_str)?;
    let l_packet_json: Value = serde_json::from_str(l_packet_str)?;

    dbg!(l_packet_json.is_array());

    // let l_packet_array = get_array(l_packet_str)?;
    dbg!(&l_packet_json);

    Ok(())
}

fn get_array(l_packet_str: &str) -> Result<Vec<i64>> {
    let l_packet_json: Value = serde_json::from_str(l_packet_str)?;
    let l_packet_array: Vec<i64> = l_packet_json
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e.as_i64().unwrap())
        .collect();

    Ok(l_packet_array)
}
