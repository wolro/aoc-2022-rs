/// Advent of Code day 14
/// https://adventofcode.com/2022/day/14
use anyhow::Result;
use eframe::egui;
use egui_plot::Plot;

#[derive(Default)]
struct P14Gui {
    input_state: String,
    current_state: String,
}

impl P14Gui {
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for P14Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Here will be a sandflow simulator.");
        });
    }
}

fn main() -> Result<()> {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();

    let mut stone_walls = Vec::new();

    for line in lines {
        let mut stone_wall = Vec::new();
        let coords = line.split(" -> ").collect::<Vec<_>>();
        for coord in coords {
            let split_coords = coord.split(",").collect::<Vec<_>>();
            stone_wall.push((
                split_coords[0].parse::<i32>().unwrap(),
                split_coords[1].parse::<i32>().unwrap(),
            ));
        }
        stone_walls.push(stone_wall);
    }

    dbg!(&stone_walls[0]);

    // let mut stone_walls_interp = Vec::new();
    for wall in stone_walls {
        let mut wall_interp = Vec::new();
        for (idx, point) in wall.clone().iter().enumerate() {
            if idx == 0 {
                wall_interp.push(point)
            } else {
                dbg!(wall[idx].0);
                dbg!(wall[idx - 1].0);
                dbg!(wall[idx].0 - wall[idx - 1].0);
                // dbg!(&wall[idx].0 - &wall[idx - 1].0);
                // dbg!(&wall[idx].1 - &wall[idx - 1].1);
            }
        }
    }

    // fire up GUI
    let mut gui_state = P14Gui::new();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1000.0, 660.0)),
        ..Default::default()
    };

    let gui = eframe::run_native(
        "Sandflow Viewer",
        options,
        Box::new(|creation_context| {
            let style = egui::Style {
                visuals: egui::Visuals::dark(),
                ..egui::Style::default()
            };
            creation_context.egui_ctx.set_style(style);
            Box::new(gui_state)
        }),
    );

    match gui {
        Ok(_res) => {}
        Err(_res) => {
            println!("Error executing GUI thread.")
        }
    };

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
