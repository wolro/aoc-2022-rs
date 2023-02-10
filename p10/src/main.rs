/// Advent of Code day 10
/// https://adventofcode.com/2022/day/10
///
/// Part 1 turned out more awkward than initially expected. There is probably
/// a much better way of exposing the register value at a given cycle count.
/// Part 2 was much more fun, I used the "plotters" crate to visualize the CRT output.
use anyhow::Result;
use plotters::prelude::*;

const CRT_LINES: i32 = 40;

/// Enum encoding the different operation types.
#[derive(Debug, Clone, Copy)]
enum Instructions {
    Noop,
    Addx(i32),
}
impl Instructions {
    fn cycles(self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_i32) => 2,
        }
    }
}

/// This models the current state of the CPU: currently executed instruction,
/// total cycle number since simulation start, current register value.
#[derive(Debug)]
struct CPUstate {
    curr_inst: Option<Instructions>,
    curr_cycle: u32,
    x: i32,
}
impl Default for CPUstate {
    fn default() -> Self {
        Self {
            curr_inst: None,
            curr_cycle: 0,
            x: 1,
        }
    }
}

/// Parses vector containing input file lines into vector with "Instructions" type entries.
fn parse_instructions(instr_list: Vec<&str>) -> Vec<Instructions> {
    let instrs = instr_list
        .iter()
        .map(|e| match *e {
            "noop" => Instructions::Noop,
            _ => {
                let (_inst, val_str) = e
                    .split_once(' ')
                    .expect("Unknown instruction in input file.");
                let val = val_str
                    .parse::<i32>()
                    .expect("Error parsing Addx instruction.");
                Instructions::Addx(val)
            }
        })
        .collect::<Vec<_>>();
    instrs
}

/// This function runs the simulation and returns the state at the end of the simulation,
/// as well as the register value at cycle numbers passed in the "probe_cycle" vector,
/// as well as a vector of tuples, each representing the coordinate of a drawn pixel.
fn simulate_cpu(
    instrs: Vec<Instructions>,
    probe_cycles: Option<Vec<u32>>,
) -> (CPUstate, Vec<i32>, Vec<(i32, i32)>) {
    let mut state: CPUstate = Default::default();
    let mut signal_hist: Vec<i32> = Vec::new();
    let mut drawn_pixels: Vec<(i32, i32)> = Vec::new();
    for instr in instrs {
        state.curr_inst = Some(instr);
        for _cyc in 0..(&state.curr_inst.clone()).unwrap().cycles() {
            let curr_cycle: i32 = (&state.curr_cycle).clone().try_into().unwrap();
            if ((curr_cycle % CRT_LINES) - &state.x).abs() < 2 {
                drawn_pixels.push((curr_cycle % CRT_LINES, (curr_cycle / CRT_LINES + 1)));
            }
            state.curr_cycle += 1;
            if probe_cycles.is_some() {
                if probe_cycles.clone().unwrap().contains(&state.curr_cycle) {
                    signal_hist.push(state.x.clone());
                }
            }
        }

        match &state.curr_inst {
            Some(Instructions::Noop) => {}
            Some(Instructions::Addx(val)) => {
                state.x += val;
            }
            _ => {
                println!("Error executing instruction.");
            }
        }
    }
    (state, signal_hist, drawn_pixels)
}

/// Display CRT image as scatter plot using "plotters" crate,
/// using a vector of coordinate tuples as input.
fn plot_crt_image(crt_image: Vec<(i32, i32)>) -> Result<()> {
    let store_str = String::from("CRT_image.png");
    let root_area = BitMapBackend::new(&store_str, (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Right, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 80)
        .set_label_area_size(LabelAreaPosition::Top, 80)
        .caption("CRT image", ("sans-serif", 18))
        .build_cartesian_2d(-1..CRT_LINES, 13..-6)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(crt_image.iter().map(|point| {
        Circle::new(
            *point,
            10,
            Into::<ShapeStyle>::into(&RGBColor(0, 0, 0)).filled(),
        )
    }))
    .unwrap();

    Ok(())
}

fn main() -> Result<()> {
    let instr_list = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let probe_cycles: Option<Vec<u32>> = Some(vec![20, 60, 100, 140, 180, 220]);
    let (_state, signal_hist, drawn_pixels) = simulate_cpu(instrs, probe_cycles.clone());

    // Thie signal strength calculation turned out quite awkwardly with "probe cycles"
    // being an option, "signal_hist" not, the different int types, and zip returning
    // references that need to be destructured...
    let signal_strength: i32 = signal_hist
        .iter()
        .zip(probe_cycles.unwrap().iter())
        .map(|(x, y)| *x * *y as i32)
        .sum();
    println!("The signal strength is {signal_strength}.");

    plot_crt_image(drawn_pixels)?;

    Ok(())
}

/// Very basic test for part 1.
#[test]
fn check_x_part1_example1() {
    let instr_list = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let (state, _signal_hist, _drawn_pixels) = simulate_cpu(instrs, None);

    assert_eq!(state.x, -1);
}

/// Test if signal strength calculation works on more elaborate example data
/// from the description of part 1.
#[test]
fn check_x_part1_example2() {
    let instr_list = include_str!("../input_test2.txt")
        .lines()
        .collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let probe_cycles: Option<Vec<u32>> = Some(vec![20, 60, 100, 140, 180, 220]);
    let (_state, signal_hist, _drawn_pixels) = simulate_cpu(instrs, probe_cycles.clone());

    let signal_strength: i32 = signal_hist
        .iter()
        .zip(probe_cycles.unwrap().iter())
        .map(|(x, y)| *x * *y as i32)
        .sum();
    assert_eq!(signal_strength, 13140);
}
