use graphs::graph::Graph;

use crate::graphs::traversal::{dfs, connected_components};

mod graphs;

fn main() {
    println!("Hello, world!");
    let graph = Graph::with_edges(
        vec![2.0, 1.0, 3.0, 2.0, 1.0, 5.0, 2.0, 1.0, 1.0, 1.0, 1.0, 3.0],
        &[
            (0, 2),
            (1, 2),
            (2, 3),
            (3, 1),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (9, 10),
            (10, 6),
            (11, 10),
        ],
    );

    println!("BFS traversal from 1:");
    dfs(&graph, 1, |x, w| println!("{} (w={})", x, w), |_, _| {});

    println!("BFS traversal from 2:");
    dfs(&graph, 2, |x, w| println!("{} (w={})", x, w), |_, _| {});

    println!("BFS traversal from 5:");
    dfs(&graph, 5, |x, w| println!("{} (w={})", x, w), |_, _| {});

    println!("Connected components: {:?}", connected_components(&graph));
}
