/// Advent of Code day 7
/// https://adventofcode.com/2022/day/7
///
/// Oh, we could resolve this using a tree. Now I know trees in Rust can be painful.
/// Digging around, it seems that "Arena-allocated Trees" are the way to go.
/// There are a bunch of crates, but to me it would seem reasonable to just use
/// "petgraph" (supporting all kinds of graphs, API looks nice)
///
/// Since we are here to learn, we will try to roll our own (see module "dirtree")!
/// Taking this article as inspiration:
/// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
mod dirtree;
use anyhow::Result;
use dirtree::DirTree;

const SPACE_TOTAL: u32 = 70000000;
const SPACE_UPDATE: u32 = 30000000;

/// Parse the input data and fill up the tree accordingly.
fn build_dirtree(lines: Vec<&str>) -> Result<DirTree> {
    // initialize tree
    let mut dir_tree: DirTree = DirTree::default();
    let root_idx = dir_tree.create_node("/".to_owned(), 0);
    let mut currdir_idx = root_idx;

    // build directory tree from input
    for log_line in lines {
        let split_line = log_line.split(' ').collect::<Vec<&str>>();
        match split_line[..] {
            // matching to slices is nifty
            ["$", "ls"] => {} // we can ignore the "ls" command
            ["dir", _] => {}  // we can ignore directory entries until we cd there
            ["$", "cd", _] => match split_line[2] {
                "/" => {} // we already took care of the root folder
                ".." => {
                    currdir_idx = dir_tree.arena[currdir_idx].parent.unwrap();
                } // go back to parent directory
                _ => {
                    let node_idx = dir_tree.create_node(split_line[2].to_owned(), 0);
                    dir_tree.arena[currdir_idx].children.push(node_idx);
                    dir_tree.arena[node_idx].parent = Some(currdir_idx);
                    currdir_idx = node_idx;
                } // We update our tree when changing to a new directory. Currently,
                           // changing to directories already present in our tree is not handled
                           // properly here and would cause issues; luckily, this doesn't seem
                           // to happen in this example.
            },
            _ => dir_tree.arena[currdir_idx].add_filesize(split_line[0].parse::<u32>().unwrap()),
        }
    }

    Ok(dir_tree)
}

fn main() -> Result<()> {
    // inline data
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    // build directory tree
    let dir_tree = build_dirtree(lines)?;
    // solution to part 1
    let sum_lt100k = get_sum_lt100k(&dir_tree)?;
    println!("Sum of all directories of size <100 kb: {}", sum_lt100k);
    // solution to part 2
    let suitable_dir_size = find_dir_to_del(&dir_tree)?;
    println!(
        "Size of the smallest directory we can delete so that the update fits on our disk: {}",
        suitable_dir_size
    );

    Ok(())
}

/// Solution to part 1: find the total size of all directories, each with sizes
/// smaller than 100 kb.
fn get_sum_lt100k(dir_tree: &DirTree) -> Result<u32> {
    let mut sum_lt100k: u32 = 0;
    for dir in &dir_tree.arena {
        let mut total_size = dir.size;
        add_subdir_sizes(dir, &mut total_size, dir_tree);
        if total_size < 100000 {
            sum_lt100k += total_size;
        }
    }
    Ok(sum_lt100k)
}

/// Solution to part 2: find the size of the smallest folder that we can delete
/// so that the update fits on the disk.
fn find_dir_to_del(dir_tree: &DirTree) -> Result<u32> {
    let mut used_space = dir_tree.arena[0].size;
    add_subdir_sizes(&dir_tree.arena[0], &mut used_space, dir_tree);

    let mut suitable_dir_sizes: Vec<u32> = Vec::new();
    for dir in &dir_tree.arena {
        let mut total_size = dir.size;
        add_subdir_sizes(dir, &mut total_size, dir_tree);
        if total_size > SPACE_UPDATE - (SPACE_TOTAL - used_space) {
            suitable_dir_sizes.push(total_size)
        }
    }
    Ok(*suitable_dir_sizes
        .iter()
        .min()
        .expect("Couldn't find suitable directory to delete."))
}

/// Function that recursively adds the size of subdirectories to the size of all files
/// in the current directory, to get the cumulative directory size.
fn add_subdir_sizes(dir: &dirtree::Node, total_size: &mut u32, dir_tree: &DirTree) {
    let subdirs = dir.children.clone();
    for subdir in subdirs {
        *total_size += dir_tree.arena[subdir].size;
        add_subdir_sizes(&dir_tree.arena[subdir], total_size, dir_tree);
    }
}

/// Check if algorithm for part 1 works on test input (see puzzle description).
#[test]
fn test_sum_lt100k_exampledata_part1() {
    // inline test data
    let lines = include_str!("../test_input.txt")
        .lines()
        .collect::<Vec<_>>();

    let dir_tree = build_dirtree(lines).unwrap();
    assert_eq!(get_sum_lt100k(&dir_tree).unwrap(), 95437);
}

/// Check if algorithm for part 2 works on test input (see puzzle description).
#[test]
fn test_find_dir_to_del_part2() {
    // inline test data
    let lines = include_str!("../test_input.txt")
        .lines()
        .collect::<Vec<_>>();

    let dir_tree = build_dirtree(lines).unwrap();
    assert_eq!(find_dir_to_del(&dir_tree).unwrap(), 24933642);
}
