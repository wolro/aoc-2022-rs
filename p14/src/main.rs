/// Advent of Code day 14
/// https://adventofcode.com/2022/day/14
mod matplotlib_cmaps;

use std::fmt;
use std::ops::Sub;

use anyhow::Result;
use eframe::{egui, epaint::vec2};
use egui_plot::{Plot, PlotImage, PlotPoint};

use crate::matplotlib_cmaps::{
    BLUERED_DATA, CIVIDIS_DATA, HELL_DATA, INFERNO_DATA, MAGMA_DATA, PLASMA_DATA, TURBO_DATA,
    VIRIDIS_DATA,
};

/// Enum for colormap selection.
#[derive(PartialEq, Clone)]
enum CMaps {
    RdBu,
    Hell,
    Inferno,
    Viridis,
    Cividis,
    Magma,
    Plasma,
    Turbo,
}

/// Implements Display trait for string representation.
impl fmt::Display for CMaps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CMaps::RdBu => write!(f, "RdBu"),
            CMaps::Hell => write!(f, "Hell"),
            CMaps::Inferno => write!(f, "Inferno"),
            CMaps::Viridis => write!(f, "Viridis"),
            CMaps::Cividis => write!(f, "Cividis"),
            CMaps::Magma => write!(f, "Magma"),
            CMaps::Plasma => write!(f, "Plasma"),
            CMaps::Turbo => write!(f, "Turbo"),
        }
    }
}

#[derive(Default)]
struct P14Gui {
    map_grid_initial: Vec<Vec<i32>>,
    map_grid_current: Vec<Vec<i32>>,
    source: (usize, usize),
    grid_offset: usize,
    cur_unit_pos: (usize, usize),
    unit_nr: usize,
    frame_ctr: usize,
    texture: Option<egui::TextureHandle>,
    texopts: egui::TextureOptions,
    cmap: Option<CMaps>,
}

impl P14Gui {
    /// Build new texture if necessary, and upload to GPU memory.
    fn build_texture(&mut self, ctx: &egui::Context) {
        let curr_grid: &Vec<Vec<i32>> = self.map_grid_current.as_ref();

        let shape = (curr_grid.len(), curr_grid[0].len());

        let size_img: [usize; 2] = [shape.0, shape.1];
        if self.cmap.is_none() {
            self.cmap = Some(CMaps::Magma);
        }

        let grad = match self.cmap.as_ref().unwrap() {
            CMaps::RdBu => BLUERED_DATA,
            CMaps::Hell => HELL_DATA,
            CMaps::Inferno => INFERNO_DATA,
            CMaps::Viridis => VIRIDIS_DATA,
            CMaps::Cividis => CIVIDIS_DATA,
            CMaps::Magma => MAGMA_DATA,
            CMaps::Plasma => PLASMA_DATA,
            CMaps::Turbo => TURBO_DATA,
        };

        let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(size_img[0] as u32, size_img[1] as u32);

        // single-threaded colormapping
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let col_val = curr_grid[x as usize][y as usize] as usize;
            *pixel = image::Rgb([
                (grad[col_val][0] * 255.0) as u8,
                (grad[col_val][1] * 255.0) as u8,
                (grad[col_val][2] * 255.0) as u8,
            ]);
        }

        let img = egui::ColorImage::from_rgb([size_img[0], size_img[1]], &imgbuf);
        let texture: &egui::TextureHandle = &ctx.load_texture("demo", img, self.texopts);
        self.texture = Some(texture.clone());
    }

    /// Build heatmap view containing info elements, axis selection, colormap selection and
    /// "DragValue" boxes for color range, aspect ratio and displayed slice. To be refactored down the line.
    fn build_heatmap_view(&mut self, ui: &mut egui::Ui) {
        ui.label("");
        ui.heading("Sandflow view");
        ui.label(String::from("Units dropped: ") + (self.unit_nr).to_string().as_ref());
        ui.separator();

        self.build_texture(ui.ctx());

        let size_img: [f32; 2] = [
            self.map_grid_initial.len() as f32,
            self.map_grid_initial[0].len() as f32,
        ];

        let tex = self.texture.clone().unwrap();
        let image = PlotImage::new(
            &tex,
            PlotPoint::new(size_img[1] / 2.0, size_img[0] / 2.0),
            vec2(size_img[1], size_img[0]),
        );

        Plot::new("heatmap demo")
            // .view_aspect(self.view_ui_ele.r_aspect)
            // .show_axes([self.view_ui_ele.show_axes, self.view_ui_ele.show_axes])
            .show(ui, |plot_ui| plot_ui.image(image.name("Image")));

        self.frame_ctr = self.unit_nr;
    }

    fn update_grid(&mut self) {
        if self.cur_unit_pos == (0, 0) {
            self.cur_unit_pos = (self.source.0 - self.grid_offset, self.source.1);
        } else {
            let (x_prev, y_prev) = self.cur_unit_pos;
            if y_prev >= self.map_grid_current[0].len() - 1 {
                return;
            }
            let (x_new, y_new) = (self.cur_unit_pos.0, self.cur_unit_pos.1 + 1);
            match self.map_grid_current[x_new][y_new] {
                0 => {
                    self.cur_unit_pos = (x_new, y_new);
                    self.map_grid_current[x_new][y_new] = 64;
                    self.map_grid_current[x_prev][y_prev] = 0;
                }
                64 | 255 => {
                    if x_new == 0 {
                        return;
                    }
                    let x_new_l = x_new - 1;
                    match self.map_grid_current[x_new_l][y_new] {
                        0 => {
                            self.cur_unit_pos = (x_new_l, y_new);
                            self.map_grid_current[x_new_l][y_new] = 64;
                            self.map_grid_current[x_prev][y_prev] = 0;
                        }
                        64 | 255 => {
                            if x_new == self.map_grid_current.len() {
                                return;
                            }
                            let x_new_r = x_new + 1;
                            match self.map_grid_current[x_new_r][y_new] {
                                0 => {
                                    self.cur_unit_pos = (x_new_r, y_new);
                                    self.map_grid_current[x_new_r][y_new] = 64;
                                    self.map_grid_current[x_prev][y_prev] = 0;
                                }
                                64 | 255 => {
                                    self.cur_unit_pos = (0, 0);
                                    self.unit_nr += 1;
                                }
                                _ => unreachable!("Something went wrong."),
                            }
                        }
                        _ => unreachable!("Something went wrong."),
                    }
                }
                _ => unreachable!("Something went wrong."),
            }
        }
    }
}

impl eframe::App for P14Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(1));
        for _idx in 0..100 {
            self.update_grid();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Here will be a sandflow simulator.");
            self.build_heatmap_view(ui);
        });
    }
}

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

fn main() -> Result<()> {
    // let lines = include_str!("../input_test.txt")
    //     .lines()
    //     .collect::<Vec<_>>();
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let source = (500, 0);

    let stone_walls = parse_input(lines);
    let all_wall_points = build_rock_coordinates(stone_walls);
    let (map_grid_initial, grid_offset) = initialize_grid(all_wall_points, source);

    // fire up GUI
    let mut gui_state = P14Gui {
        map_grid_initial: map_grid_initial.clone(),
        map_grid_current: map_grid_initial,
        source,
        grid_offset,
        texopts: egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
        },
        cmap: Some(CMaps::Magma),
        ..Default::default()
    };

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

    // for _cntr in 0..200000 {
    //     let last_unit = gui_state.unit_nr;
    //     gui_state.update_grid();
    //     if gui_state.unit_nr > last_unit {
    //         print!("{}, ", gui_state.unit_nr);
    //     }
    // }

    Ok(())
}

fn initialize_grid(
    all_wall_points: Vec<Point>,
    source_coord: (usize, usize),
) -> (Vec<Vec<i32>>, usize) {
    let x_min = all_wall_points.clone().iter().map(|e| e.x).min().unwrap();
    let x_max = all_wall_points.clone().iter().map(|e| e.x).max().unwrap();
    let y_max = all_wall_points.clone().iter().map(|e| e.y).max().unwrap();

    // initialize grid (ndarray would be better):
    let mut map_grid = Vec::new();
    for _x_idx in x_min..=x_max {
        map_grid.push(vec![0; (y_max + 1) as usize]);
    }

    // populate grid with stones
    for grid_point in all_wall_points {
        map_grid[(grid_point.x - x_min) as usize][grid_point.y as usize] = 255;
    }

    let grid_offset = x_min as usize;
    map_grid[source_coord.0 - grid_offset][source_coord.1] = 128;

    (map_grid, grid_offset)
}

fn build_rock_coordinates(stone_walls: Vec<Vec<Point>>) -> Vec<Point> {
    let mut all_wall_points: Vec<Point> = Vec::new();
    for current_wall in stone_walls {
        for diff_idx in 1..current_wall.len() {
            let p_diff = current_wall[diff_idx] - current_wall[diff_idx - 1];

            all_wall_points.push(current_wall[diff_idx - 1]);

            if p_diff.x == 0 {
                for d_idx in 0..p_diff.y.abs() {
                    if d_idx > 0 {
                        all_wall_points.push(Point {
                            x: current_wall[diff_idx - 1].x,
                            y: current_wall[diff_idx - 1].y + d_idx * p_diff.y.signum(),
                        });
                    }
                }
            } else if p_diff.y == 0 {
                for d_idx in 0..p_diff.x.abs() {
                    if d_idx > 0 {
                        all_wall_points.push(Point {
                            x: current_wall[diff_idx - 1].x + d_idx * p_diff.x.signum(),
                            y: current_wall[diff_idx - 1].y,
                        });
                    }
                }
            }
        }
        all_wall_points.push(current_wall.last().unwrap().to_owned());
    }
    all_wall_points
}

fn parse_input(lines: Vec<&str>) -> Vec<Vec<Point>> {
    let mut stone_walls = Vec::new();

    for line in lines {
        let mut stone_wall: Vec<Point> = Vec::new();
        let coords = line.split(" -> ").collect::<Vec<_>>();
        for coord in coords {
            let split_coords = coord.split(',').collect::<Vec<_>>();
            stone_wall.push(Point {
                x: split_coords[0].parse::<i32>().unwrap(),
                y: split_coords[1].parse::<i32>().unwrap(),
            });
        }
        stone_walls.push(stone_wall);
    }
    stone_walls
}

#[test]
fn part1_validate_on_testdata() {
    unimplemented!();
}

#[test]
fn part2_validate_on_testdata() {
    unimplemented!();
}
