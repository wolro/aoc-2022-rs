/// Advent of Code day 15
/// https://adventofcode.com/2022/day/15
use std::fmt;
use std::ops::Sub;

use anyhow::Result;
use itertools::chain;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
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

    dbg!(&cbeacon);
    Ok((sensors, cbeacon))
}

fn initialize_grid(sensors: Vec<Point>, cbeacs: Vec<Point>) -> (Vec<Vec<u8>>, (i32, i32)) {
    let x_min = chain(
        sensors.clone().iter().map(|e| e.x),
        cbeacs.clone().iter().map(|e| e.x),
    )
    .min()
    .unwrap();

    let x_max = chain(
        sensors.clone().iter().map(|e| e.x),
        cbeacs.clone().iter().map(|e| e.x),
    )
    .max()
    .unwrap();

    let y_min = chain(
        sensors.clone().iter().map(|e| e.y),
        cbeacs.clone().iter().map(|e| e.y),
    )
    .min()
    .unwrap();

    let y_max = chain(
        sensors.clone().iter().map(|e| e.y),
        cbeacs.clone().iter().map(|e| e.y),
    )
    .max()
    .unwrap();

    // initialize grid (ndarray would be better):
    let mut map_grid: Vec<Vec<u8>> = Vec::new();
    for _x_idx in x_min..=x_max {
        map_grid.push(vec![0; (y_max + 1) as usize]);
    }

    // dbg!(&x_min);
    // dbg!(&x_max);
    // dbg!(&y_min);
    // dbg!(&y_max);
    // dbg!(&sensors);

    // populate grid with stones
    for grid_point in sensors {
        map_grid[(grid_point.x - x_min) as usize][(grid_point.y - y_min) as usize] = 1;
    }
    for grid_point in cbeacs {
        map_grid[(grid_point.x - x_min) as usize][(grid_point.y - y_min) as usize] = 2;
    }

    let grid_offsets = (x_min, y_min);

    (map_grid, grid_offsets)
}

fn main() -> Result<()> {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let (sensors, beacons) = parse_input(lines)?;
    let (mut map_grid, grid_offsets) = initialize_grid(sensors.clone(), beacons.clone());

    for sensor in sensors {
        let closest_beac_dist = beacons.iter().map(|b| (sensor - *b).norm()).min().unwrap();
        for x_idx in 0..map_grid.len() {
            for y_idx in 0..map_grid[0].len() {
                if ((Point {
                    x: (x_idx as i32 - grid_offsets.0),
                    y: (y_idx as i32 - grid_offsets.1),
                } - sensor)
                    .norm()
                    < closest_beac_dist)
                    && (map_grid[x_idx][y_idx] == 0)
                {
                    map_grid[x_idx][y_idx] = 3;
                }
            }
        }
    }

    let row_idx = 10;
    let mut no_bcn_ctr = 0;

    dbg!(map_grid[0][row_idx]);
    dbg!(map_grid.last().expect("Vector has no last element?")[row_idx]);

    for checked_pos in 0..map_grid.len() {
        dbg!(map_grid[checked_pos][row_idx]);
        if map_grid[checked_pos][row_idx] > 0 {
            no_bcn_ctr += 1
        } else {
        };
    }

    dbg!(&no_bcn_ctr);

    Ok(())
}

#[test]
fn part1_validate_on_testdata() {
    unimplemented!();
}

#[test]
fn part2_validate_on_testdata() {
    unimplemented!();
}
