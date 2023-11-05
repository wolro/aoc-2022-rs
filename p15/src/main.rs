/// Advent of Code day 15
/// https://adventofcode.com/2022/day/15
///
/// This was interesting, and I had no issues to find a solution for part 1 that was in principle right,
/// but I had a hard time debugging off-by-one errors for ranges. Started out with a grid, which
/// worked fine for the test data, but fell flat for the real data due to the grid size.
/// Changed the approach to end up with a slow but fast enough solution.
///
/// Similar story for part 2: Brute force approach worked fine for test data, but was hopeless
/// for the real input data. At this point I already had stumbled upon some spoilers on how other
/// people solved this, so I went with the simplest approach: checking if there are any spots at
/// sensor-beacon distance + 1. This is fairly slow (37s) but at this point I just I am very much
/// over this day's puzzle and want to get on, so: neither optimization nor cleanup of the code :-)
use std::collections::HashSet;
use std::ops::Sub;
use std::time::Instant;

use anyhow::Result;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
/// Implement subtraction operation for our "Point".
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
    /// This, applied to the difference between two points, will yield the Manhattan
    /// distance between said points.
    fn norm(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

/// The obligatory parser.
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

    Ok((sensors, cbeacon))
}

/// Solution for part 1; Check for points in within sensor range within specified row.
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

/// Brute force approach to part 2; only used in a test case since hopeless for real input data.
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

/// Get points just outside sensor range.
fn border_points(sensors: Vec<Point>, beacons: Vec<Point>, gridsize: (i32, i32)) -> HashSet<Point> {
    let mut candidates = HashSet::new();

    for (idx, sensor) in sensors.iter().enumerate() {
        let border = (*sensor - beacons[idx]).norm() as i32;
        let outside_border = border + 1;

        for rp_idx in 0..outside_border {
            let candidate1 = Point {
                x: sensor.x + outside_border - rp_idx,
                y: sensor.y + rp_idx,
            };
            check_borders_and_add(candidate1, &mut candidates, gridsize);

            let candidate2 = Point {
                x: sensor.x + outside_border - rp_idx,
                y: sensor.y - rp_idx,
            };
            check_borders_and_add(candidate2, &mut candidates, gridsize);

            let candidate3 = Point {
                x: sensor.x - outside_border + rp_idx,
                y: sensor.y + rp_idx,
            };
            check_borders_and_add(candidate3, &mut candidates, gridsize);

            let candidate4 = Point {
                x: sensor.x - outside_border + rp_idx,
                y: sensor.y - rp_idx,
            };
            check_borders_and_add(candidate4, &mut candidates, gridsize);
        }
    }

    dbg!(&candidates.len());
    candidates
}

/// Helper function to get rid of candidates outside the specified grid size.
fn check_borders_and_add(candidate: Point, candidates: &mut HashSet<Point>, gridsize: (i32, i32)) {
    if (candidate.x >= gridsize.0)
        && (candidate.x <= gridsize.1)
        && (candidate.y >= gridsize.0)
        && (candidate.y <= gridsize.1)
    {
        candidates.insert(candidate);
    }
}

/// Checks if "candidates" (points just one distance unit outside sensor range
/// as provided by "border_points") are within range of any other sensor, and
/// returns any identified out-of-range points.
fn beacon_candidates(
    candidates: HashSet<Point>,
    sensors: Vec<Point>,
    beacons: Vec<Point>,
) -> Vec<Point> {
    let mut excluded_candidates: HashSet<Point> = HashSet::new();

    for candidate in &candidates {
        for (idx, sensor) in sensors.iter().enumerate() {
            if (*sensor - *candidate).norm() <= (*sensor - beacons[idx]).norm() {
                excluded_candidates.insert(*candidate);
            }
        }
    }

    let remaining_candidates = candidates
        .difference(&excluded_candidates)
        .cloned()
        .collect::<Vec<_>>();
    remaining_candidates
}

/// Calculate tuning frequency for part 2 solution.
fn tuning_frequency(beacon_candidate: Point) -> u64 {
    beacon_candidate.x as u64 * 4000000 + beacon_candidate.y as u64
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
    println!("Part 2:");
    let candidates = border_points(sensors.clone(), beacons.clone(), (0, 4000000));
    let remaining_candidates = beacon_candidates(candidates, sensors, beacons);
    dbg!(&remaining_candidates[0]);
    let tuning_freq = tuning_frequency(remaining_candidates[0]);
    dbg!(&tuning_freq);
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

#[test]
fn part2_validate_on_testdata_v2() {
    const TUNING_FREQ_REF: u64 = 56000011;

    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let (sensors, beacons) = parse_input(lines).unwrap();

    let candidates = border_points(sensors.clone(), beacons.clone(), (0, 20));
    let remaining_candidates = beacon_candidates(candidates, sensors, beacons);
    let tuning_freq = tuning_frequency(remaining_candidates[0]);

    assert_eq!(tuning_freq, TUNING_FREQ_REF);
}
