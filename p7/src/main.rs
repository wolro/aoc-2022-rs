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

fn main() {
    let lines = include_str!("../test_input.txt").lines().collect::<Vec<_>>();
}
