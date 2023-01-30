use anyhow::Result;

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let mut int_lines: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut int_line: Vec<u32> = Vec::new();
        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                int_line.push(digit);
            }
        }
        int_lines.push(int_line);
    }

    let mut int_cols: Vec<Vec<u32>> = Vec::new();
    for col in 0..int_lines[0].len() {
        let mut int_col: Vec<u32> = Vec::new();
        for line in 0..int_lines.len() {
            int_col.push(int_lines[line][col])
        }
        int_cols.push(int_col);
    }

    let grid_shape = (
        int_lines.len() as u32,
        int_cols.len() as u32,
    );

    // solution for part 1
    let mut vis_cnt: u32 = 0;
    for tree_x in 1..&int_cols.len()-1 {
        for tree_y in 1..&int_lines.len()-1 {
            if is_visible(&int_lines, tree_x, tree_y) || is_visible(&int_cols, tree_y, tree_x) {
                vis_cnt += 1;
            }
        }
    }
    vis_cnt = vis_cnt + 2*grid_shape.0 + 2*grid_shape.1 - 4;
    dbg!(vis_cnt);
}

fn is_visible(lines: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let reduced_line = lines[x].clone();
    let tree_size = reduced_line[y];
    // reduced_line.remove(y);
    let v_one = reduced_line[..y]
        .iter()
        .any(|&idx| idx >= tree_size);
    let v_two = reduced_line[y+1..]
        .iter()
        .any(|&idx| idx >= tree_size);
    // dbg!(&x); dbg!(&y); dbg!(&tree_size);
    !v_one || !v_two
}
