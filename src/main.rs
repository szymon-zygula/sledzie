use graphs::graph::Graph;

use crate::graphs::traversal::{BFS, GraphTraversal};

mod graphs;

fn main() {
    println!("Hello, world!");
    let mut g = Graph::new(vec![2.0, 1.0, 3.0, 2.0, 1.0, 5.0, 2.0, 1.0, 1.0, 1.0, 1.0, 3.0]);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 2);
    g.add_edge(5, 6);
    g.add_edge(6, 7);
    g.add_edge(7, 8);
    g.add_edge(8, 9);
    g.add_edge(9, 10);
    g.add_edge(10, 11);
    g.add_edge(11, 8);
    g.add_edge(12, 7);

    println!("BFS traversal from 1:");
    BFS::traverse(&g, 1, &mut |x, w| println!("{} (w={})", x, w));

    println!("BFS traversal from 2:");
    BFS::traverse(&g, 2, &mut |x, w| println!("{} (w={})", x, w));

    println!("BFS traversal from 5:");
    BFS::traverse(&g, 5, &mut |x, w| println!("{} (w={})", x, w));
}
