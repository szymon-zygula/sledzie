use crate::graphs::graph::Graph;

pub fn dfs<En: FnMut(usize, f64), Ex: FnMut(usize, f64)>(
    graph: &Graph,
    start_vertex: usize,
    mut on_enter: En,
    mut on_exit: Ex,
) {
    let mut visited = vec![false; graph.vertex_count()];
    let mut queue = Vec::new();
    queue.push(start_vertex);
    visited[start_vertex] = true;

    while let Some(vertex) = queue.pop() {
        on_enter(vertex, graph.weight(vertex));

        for neighbor in graph.children(vertex) {
            if !visited[neighbor] {
                queue.push(neighbor);
                visited[neighbor] = true;
            }
        }

        on_exit(vertex, graph.weight(vertex));
    }
}

/// Returns a `Vec` of vertex indices with exactly one vertex for each connected component in
/// `graph`.
pub fn connected_components(graph: &Graph) -> Vec<usize> {
    let mut visited = vec![false; graph.vertex_count()];
    let mut queue = Vec::new();
    let mut components = Vec::new();

    for start_vertex in 0..graph.vertex_count() {
        if visited[start_vertex] {
            continue;
        }

        queue.push(start_vertex);
        visited[start_vertex] = true;
        components.push(start_vertex);

        while let Some(vertex) = queue.pop() {
            for neighbor in graph.neighbors(vertex) {
                if !visited[neighbor] {
                    queue.push(neighbor);
                    visited[neighbor] = true;
                }
            }
        }
    }

    components
}

/// Checks if `graph` contains a cycle. If it does not, returns `None`.
/// If it does, returns `Some(edge)`, where `edge` is an arbitrary edge in a cycle in `graph`.
/// `component` is any vertex of the connected component in which to look for the cycle.
///
pub fn cycle_edge(graph: &Graph, component: usize) -> Option<(usize, usize)> {
    let mut visited = vec![true; graph.vertex_count()];
    let mut queue = Vec::new();
    queue.push(component);
    visited[component] = true;

    while let Some(vertex) = queue.pop() {
        for neighbor in graph.neighbors(vertex) {
            if visited[neighbor] {
                return Some((vertex, neighbor));
            }

            queue.push(neighbor);
            visited[neighbor] = true;
        }
    }

    None
}

/// Returns a `Vec` of vertices which are leaves in the connected component containing `component`
/// vertex
pub fn leaves(graph: &Graph, component: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.vertex_count()];
    let mut queue = Vec::new();
    let mut leaves = Vec::new();

    queue.push(component);
    visited[component] = true;

    while let Some(vertex) = queue.pop() {
        if graph.children(vertex).is_empty() {
            leaves.push(vertex);
        }

        for neighbor in graph.neighbors(vertex) {
            if !visited[neighbor] {
                queue.push(neighbor);
                visited[neighbor] = true;
            }
        }
    }

    leaves
}
