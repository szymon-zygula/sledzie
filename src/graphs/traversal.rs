use crate::graphs::graph::Graph;

pub trait GraphTraversal {
    fn traverse(graph: &Graph, start_vertex: usize, vertex_callback: &mut dyn FnMut(usize, f64));
}

pub struct BFS;

impl GraphTraversal for BFS {
    fn traverse(graph: &Graph, start_vertex: usize, vertex_callback: &mut dyn FnMut(usize, f64)) {
        let mut visited = vec![false; graph.vertex_count() + 1];
        let mut queue = Vec::new();
        queue.push(start_vertex);
        visited[start_vertex] = true;
        while !queue.is_empty() {
            let vertex = queue.remove(0);
            vertex_callback(vertex, graph.weight(vertex));
            for &neighbor in graph.neighbors(vertex) {
                if !visited[neighbor] {
                    queue.push(neighbor);
                    visited[neighbor] = true;
                }
            }
        }
    }
}
