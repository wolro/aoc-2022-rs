use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    node: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &HashMap<i64, Vec<(i64, i64)>>, start: i64, end: i64) -> Option<i64> {
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, node: start });
    let mut distances: HashMap<i64, i64> = HashMap::new();
    distances.insert(start, 0);
    while let Some(State { cost, node }) = heap.pop() {
        if node == end {
            return Some(cost);
        }
        if cost > distances[&node] {
            continue;
        }
        for &(neighbour, cost) in graph.get(&node).unwrap().iter() {
            let new_cost = distances[&node] + cost;
            if distances.contains_key(&neighbour) {
                if new_cost < distances[&neighbour] {
                    distances.insert(neighbour, new_cost);
                    heap.push(State { cost: new_cost, node: neighbour });
                }
            } else {
                distances.insert(neighbour, new_cost);
                heap.push(State { cost: new_cost, node: neighbour });
            }
        }
    }
    None
}

fn main() {
    let mut graph: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    graph.insert(1, vec![(2, 1), (3, 4), (4, 2)]);
    graph.insert(2, vec![(4, 1)]);
    graph.insert(3, vec![(4, 5)]);
    graph.insert(4, vec![]);
    let start = 1;
    let end = 4;
    let distance = dijkstra(&graph, start, end);
    if let Some(d) = distance {
        println!("Shortest distance from node {} to node {} is {}", start, end, d);
    } else {
        println!("No path from node {} to node {}", start, end);
    }
}