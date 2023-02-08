/// Advent of Code day 10
/// https://adventofcode.com/2022/day/10
/// 
/// This turned out more awkward than initially expected. There is probably
/// a much better way of exposing the register value at a given cycle count.

use anyhow::Result;

/// Enum encoding the different operation types.
#[derive(Debug, Clone, Copy)]
enum Instructions {
    Noop,
    Addx(i32)
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
    let instrs = instr_list.iter()
        .map(|e| {
            match *e {
                "noop" => Instructions::Noop,
                _ => {
                    let (_inst, val_str) = e.split_once(' ').expect("Unknown instruction in input file.");
                    let val = val_str.parse::<i32>().expect("Error parsing Addx instruction.");
                    Instructions::Addx(val)
                }
            }
        })
        .collect::<Vec<_>>();
    instrs
}

/// This function runs the simulation and returns the state at the end of the simulation,
/// as well as the register value at cycle numbers passed in the "probe_cycle" vector.
fn simulate_cpu(instrs: Vec<Instructions>, probe_cycles: Option<Vec<u32>>) -> (CPUstate, Vec<i32>) {
    let mut state: CPUstate = Default::default();
    let mut signal_hist: Vec<i32> = Vec::new(); 
    for instr in instrs {
        state.curr_inst = Some(instr);
        for _cyc in 0..(&state.curr_inst.clone()).unwrap().cycles() {
            state.curr_cycle +=1; 
            if probe_cycles.is_some() {
                if probe_cycles.clone().unwrap().contains(&state.curr_cycle) {
                    
                    signal_hist.push(state.x.clone());
                }
            }
        }
    
        match &state.curr_inst {
            Some(Instructions::Noop) => {},
            Some(Instructions::Addx(val)) => { state.x += val; },
            _ => {println!("Error executing instruction."); },
        }
    }
    (state, signal_hist)
}

fn main() -> Result<()> {
    let instr_list = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let probe_cycles: Option<Vec<u32>> = Some(vec![20, 60, 100, 140, 180, 220]);
    let (_state, signal_hist) = simulate_cpu(instrs, probe_cycles.clone());

    // Thie signal strength calculation turned out quite awkwardly with "probe cycles"
    // being an option, "signal_hist" not, the different int types, and zip returning
    // references that need to be destructured...
    let signal_strength: i32 = signal_hist.iter().zip(probe_cycles.unwrap().iter()).map(|(x,y)| *x * *y as i32).sum();    
    println!("The signal strength is {signal_strength}.");
    
    Ok(())    
}

/// Very basic test for part 1.
#[test]
fn check_x_part1_example1() {
    let instr_list = include_str!("../input_test.txt").lines().collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let (state, _signal_hist) = simulate_cpu(instrs, None);

    assert_eq!(state.x, -1);
}

/// Test if signal strength calculation works on more elaborate example data
/// from the description of part 1.
#[test]
fn check_x_part1_example2() {
    let instr_list = include_str!("../input_test2.txt").lines().collect::<Vec<_>>();

    let instrs = parse_instructions(instr_list);
    let probe_cycles: Option<Vec<u32>> = Some(vec![20, 60, 100, 140, 180, 220]);
    let (_state, signal_hist) = simulate_cpu(instrs, probe_cycles.clone());
    
    let signal_strength: i32 = signal_hist.iter().zip(probe_cycles.unwrap().iter()).map(|(x,y)| *x * *y as i32).sum();    
    assert_eq!(signal_strength,13140);   
}

