/// Advent of Code day 8
/// https://adventofcode.com/2022/day/8
///
/// Helping the elves with their tree house was straightforward enough.
/// This solution is probably not very elegant (one could probably
/// define a proper grid type with its own iterator to traverse it),
/// but it worked without having to debug a whole lot.
use anyhow::Result;

/// parse forest into two vectors represending grid lines and columns, respectively.
fn get_grid_vecs(lines: Vec<&str>) -> Result<(Vec<Vec<u32>>, Vec<Vec<u32>>)> {
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
    Ok((int_lines, int_cols))
}

/// Determine if a tree with grid coordinates (coord_1, coord_2) is visible from outside the grid.
fn is_visible(lines: &Vec<Vec<u32>>, coord_1: usize, coord_2: usize) -> Result<bool> {
    let reduced_line = lines[coord_1].clone();
    let tree_size = reduced_line[coord_2];

    let v_one = reduced_line[..coord_2].iter().any(|&idx| idx >= tree_size); // "left" or "upper" side with respect to current tree
    let v_two = reduced_line[coord_2 + 1..] // "right" or "lower" side with respect ot current tree
        .iter()
        .any(|&idx| idx >= tree_size);

    Ok(!v_one || !v_two)
}

/// Calculate number of visible trees.
fn nr_visible_trees(int_cols: &Vec<Vec<u32>>, int_lines: &Vec<Vec<u32>>) -> Result<u32> {
    let mut vis_cnt: u32 = 0;
    for tree_x in 1..&int_cols.len() - 1 {
        for tree_y in 1..&int_lines.len() - 1 {
            if is_visible(int_lines, tree_x, tree_y)? || is_visible(int_cols, tree_y, tree_x)? {
                vis_cnt += 1;
            }
        }
    }
    let grid_shape = (int_lines.len() as u32, int_cols.len() as u32);

    vis_cnt = vis_cnt + 2 * grid_shape.0 + 2 * grid_shape.1 - 4;
    Ok(vis_cnt)
}

/// Find grid distance of closest trees bigger than or as big as the tree
/// with grid coordinates (coord_1, coord_2) along horizontal OR vertical direction,
/// depending on input parameters
fn viewblockers(lines: &Vec<Vec<u32>>, coord_1: usize, coord_2: usize) -> (usize, usize) {
    let reduced_line = lines[coord_1].clone();
    let tree_size = reduced_line[coord_2];

    let v_one = reduced_line[..coord_2]
        .iter()
        .rev()
        .position(|&idx| idx >= tree_size);
    let v_two = reduced_line[coord_2 + 1..]
        .iter()
        .position(|&idx| idx >= tree_size);

    let one = match v_one {
        Some(e) => e + 1, // elves index 1-based, not 0-based :-)
        None => coord_2,
    };
    let two = match v_two {
        Some(e) => e + 1,
        None => reduced_line.len() - coord_2 - 1,
    };

    (one, two)
}

/// Calculate scenic score from distances determined by the "viewblockers()" function.
fn get_scenic_score(
    int_cols: &Vec<Vec<u32>>,
    int_lines: &Vec<Vec<u32>>,
    coord_1: usize,
    coord_2: usize,
) -> Result<u32> {
    // vertical grid direction
    let (a, b) = viewblockers(&int_cols, coord_1, coord_2);
    // horizontal grid direction
    let (c, d) = viewblockers(&int_lines, coord_2, coord_1);

    Ok((a * b * c * d).try_into().unwrap())
}

fn main() -> Result<()> {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let (int_lines, int_cols) = get_grid_vecs(lines)?;

    // solution for part 1
    let vis_cnt = nr_visible_trees(&int_cols, &int_lines)?;
    println!("The number of visible trees is: {}", vis_cnt);

    // solution for part 2
    let mut max_scenic_score: u32 = 0;
    for tree_x in 1..&int_cols.len() - 1 {
        for tree_y in 1..&int_lines.len() - 1 {
            let current_score = get_scenic_score(&int_lines, &int_cols, tree_x, tree_y)?;
            if current_score > max_scenic_score {
                max_scenic_score = current_score;
            }
        }
    }
    println!("The best tree has a scenic score of: {}", max_scenic_score);

    Ok(())
}

/// Test for part 1.
#[test]
fn check_find_visible_trees_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let (int_lines, int_cols) = get_grid_vecs(lines).unwrap();
    assert_eq!(nr_visible_trees(&int_cols, &int_lines).unwrap(), 21)
}

/// Test for part 2.
#[test]
fn check_find_viewblockers_on_testdata() {
    let lines = include_str!("../input_test.txt")
        .lines()
        .collect::<Vec<_>>();
    let (int_lines, int_cols) = get_grid_vecs(lines).unwrap();

    let (a, b) = viewblockers(&int_cols, 2, 1);
    let (c, d) = viewblockers(&int_lines, 1, 2);
    assert_eq!(a * b * c * d, 4);

    let (a, b) = viewblockers(&int_cols, 2, 3);
    let (c, d) = viewblockers(&int_lines, 3, 2);
    assert_eq!(a * b * c * d, 8);
}
