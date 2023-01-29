/// Advent of Code day 7
/// https://adventofcode.com/2022/day/7
///
/// Oh, we have to deal with a tree. Now I know trees in Rust can be painful.
/// Digging around, it seems that "Arena-Allocated Trees" are the way to go.
/// There are a bunch of crates, but to me it seems reasonable to just use
/// "petgraph" (supporting all kinds of graphs, API looks nice)
///
/// Since we are here to learn, we will try to roll our own! Taking this
/// article as inspiration:
/// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

mod dirtree;
use dirtree::DirTree;
use anyhow::Result;


fn build_dirtree(lines: Vec<&str>) -> Result<DirTree>{
    // initialize tree
    let mut dir_tree: DirTree = DirTree::default();
    let root_idx  = dir_tree.node("/".to_owned(), 0);
    let mut currdir_idx = root_idx;

    // build directory tree from input
    for log_line in lines {
        let split_line = log_line.split(' ').collect::<Vec<&str>>();
        match split_line [..]{
            ["$", "ls"] => {},
            ["dir", _] => {},
            ["$", "cd", _ ] => {
                match split_line[2] {
                    "/" => {},
                    ".." => {
                        currdir_idx = dir_tree.arena[currdir_idx].parent.unwrap();
                    },
                    _ => {
                            let node_idx  = dir_tree.node(split_line[2].to_owned(), 0);
                            dir_tree.arena[currdir_idx].children.push(node_idx);
                            dir_tree.arena[node_idx].parent = Some(currdir_idx);
                            currdir_idx = node_idx;
                        },
                    }

                },
            _ => { dir_tree.arena[currdir_idx].add_filesize(split_line[0].parse::<u32>().unwrap()) },
        }
    }

    Ok(dir_tree)
}


fn main() -> Result<()> {
    // read data
    let lines = include_str!("../test_input.txt")
        .lines()
        .collect::<Vec<_>>();

    let dir_tree = build_dirtree(lines);
    dbg!(&dir_tree);

    Ok(())
}

