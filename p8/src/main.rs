use anyhow::Result;

fn main() {
    let lines = include_str!("../input_test.txt").lines().collect::<Vec<_>>();

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

    dbg!(&int_lines[0]);
    let mut reduced_line = int_lines[0].clone();
    reduced_line.remove(3);
    dbg!(&reduced_line);
    let hlol = reduced_line
        .iter()
        .any(|&x| x >= 7);
    dbg!(hlol);

    dbg!(&int_cols[3]);
    let mut reduced_col = int_cols[3].clone();
    reduced_col.remove(0);
    dbg!(&reduced_col);
    let vlol = reduced_col
        .iter()
        .any(|&x| x >= 7);
    dbg!(vlol);
}
