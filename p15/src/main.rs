/// Advent of Code day 15
/// https://adventofcode.com/2022/day/15
use std::collections::HashSet;
use std::ops::Sub;
use std::time::Instant;

use anyhow::Result;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn norm(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

fn parse_input(lines: Vec<&str>) -> Result<(Vec<Point>, Vec<Point>)> {
    let mut sensors = Vec::new();
    let mut cbeacon = Vec::new();

    for line in lines {
        let (sens_entry, beac_entry) = line
            .split_once(':')
            .expect("Input line didn't contain colon?");
        let x_sens_borders = (
            sens_entry.find('x').unwrap() + 2,
            sens_entry.find(',').unwrap(),
        );
        let y_sens_borders = (sens_entry.find('y').unwrap() + 2, sens_entry.len());
        let x_beac_borders = (
            beac_entry.find('x').unwrap() + 2,
            beac_entry.find(',').unwrap(),
        );
        let y_beac_borders = (beac_entry.find('y').unwrap() + 2, beac_entry.len());

        sensors.push(Point {
            x: sens_entry[x_sens_borders.0..x_sens_borders.1].parse::<i32>()?,
            y: sens_entry[y_sens_borders.0..y_sens_borders.1].parse::<i32>()?,
        });

        cbeacon.push(Point {
            x: beac_entry[x_beac_borders.0..x_beac_borders.1].parse::<i32>()?,
            y: beac_entry[y_beac_borders.0..y_beac_borders.1].parse::<i32>()?,
        });
    }

    // dbg!(&sensors);
    Ok((sensors, cbeacon))
}

fn excluded_positions(sensors: Vec<Point>, beacons: Vec<Point>, line_idx: i32) -> i32 {
    let mut ex_pos = HashSet::new();

    for (idx, sensor) in sensors.iter().enumerate() {
        let closest_beac_dist = (*sensor - beacons[idx]).norm();
        let x_range =
            (sensor.x - (closest_beac_dist as i32))..(sensor.x + (closest_beac_dist as i32));
        for x_idx in x_range {
            let candidate = Point {
                x: x_idx,
                y: line_idx,
            };
            if ((candidate - *sensor).norm() <= closest_beac_dist)
                && !(beacons.contains(&candidate))
            {
                ex_pos.insert(candidate);
            }
        }
    }
    ex_pos.len() as i32
}

fn beacon_position(
    sensors: Vec<Point>,
    beacons: Vec<Point>,
    line_idx: i32,
    x_max: i32,
) -> Option<Vec<Point>> {
    let mut all_pos = HashSet::new();
    let mut ex_pos = HashSet::new();
    // let mut beac_pos: Option<Vec<Point>> = None;

    for (idx, sensor) in sensors.iter().enumerate() {
        let closest_beac_dist = (*sensor - beacons[idx]).norm();
        for x_idx in 0..x_max {
            let candidate = Point {
                x: x_idx,
                y: line_idx,
            };
            if !(beacons.contains(&candidate)) {
                all_pos.insert(candidate);
            }
            if ((candidate - *sensor).norm() <= closest_beac_dist)
                && !(beacons.contains(&candidate))
            {
                // dbg!(&candidate);
                ex_pos.insert(candidate);
            }
        }
    }
    let beac_pos_set = all_pos.difference(&ex_pos);
    Some(beac_pos_set.into_iter().copied().collect())
}

fn main() -> Result<()> {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let (sensors, beacons) = parse_input(lines)?;

    let before_part1 = Instant::now();
    let row_idx = 2000000;
    let no_bcn_ctr = excluded_positions(sensors.clone(), beacons.clone(), row_idx);
    println!("Part 1:");
    dbg!(&no_bcn_ctr);
    println!("Elapsed time: {:.2?}", before_part1.elapsed());

    let before_part2 = Instant::now();
    let x_max: i32 = 1;
    let mut beac_pos = Point { x: 0, y: 0 };
    for row_idx in 0..4000000 {
        if let Some(points) = beacon_position(sensors.clone(), beacons.clone(), row_idx, x_max) {
            if !points.is_empty() {
                beac_pos = points[0];
            }
        }
    }
    println!("Part 2:");
    dbg!(beac_pos);
    println!("Elapsed time: {:.2?}", before_part2.elapsed());

    Ok(())
}

#[test]
fn part1_validate_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let (sensors, beacons) = parse_input(lines).unwrap();

    let row_idx = 10;
    let no_bcn_ctr = excluded_positions(sensors, beacons, row_idx);

    assert_eq!(no_bcn_ctr, 26);
}

#[test]
fn part2_validate_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let (sensors, beacons) = parse_input(lines).unwrap();

    let x_max: i32 = 20;
    let mut beac_pos = Point { x: 0, y: 0 };

    for row_idx in 0..20 {
        if let Some(points) = beacon_position(sensors.clone(), beacons.clone(), row_idx, x_max) {
            if !points.is_empty() {
                beac_pos = points[0];
            }
        }
    }

    let beac_pos_ref = Point { x: 14, y: 11 };

    assert_eq!(beac_pos, beac_pos_ref);
}
