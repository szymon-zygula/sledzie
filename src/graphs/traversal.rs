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

        for neighbor in graph.following(vertex) {
            if !visited[neighbor] {
                queue.push(neighbor);
                visited[neighbor] = true;
            }
        }

        on_exit(vertex, graph.weight(vertex));
    }
}

#[derive(Clone, Debug)]
pub struct ConnectedComponents {
    pub graphs: Vec<Graph>,
    pub name_mapping: Vec<Vec<usize>>,
}

impl ConnectedComponents {
    /// Returns a `Vec` of vertex indices with exactly one vertex for each connected component in
    /// `graph`.
    pub fn new(graph: &Graph) -> Self {
        let mut component_marks = vec![None; graph.vertex_count()];
        let mut queue = Vec::new();
        let mut components: Vec<Vec<usize>> = Vec::new();
        let mut new_names = vec![0; graph.vertex_count()];

        for start_vertex in 0..graph.vertex_count() {
            if component_marks[start_vertex].is_some() {
                continue;
            }

            queue.push(start_vertex);
            component_marks[start_vertex] = Some(components.len());
            components.push(vec![start_vertex]);
            new_names[start_vertex] = 0;

            while let Some(vertex) = queue.pop() {
                for neighbor in graph.neighbors(vertex) {
                    if component_marks[neighbor].is_none() {
                        queue.push(neighbor);

                        component_marks[neighbor] = Some(components.len() - 1);
                        new_names[neighbor] = components.last().unwrap().len();
                        components.last_mut().unwrap().push(neighbor);
                    }
                }
            }
        }

        let mut component_graphs = Vec::new();
        let mut name_mapping = Vec::new();

        for component in components {
            let mut component_graph =
                Graph::new(component.iter().map(|&v| graph.weight(v)).collect());
            let mut original_names = vec![0; component_graph.vertex_count()];

            for v in component {
                original_names[new_names[v]] = v;
                for followed in graph.following(v) {
                    component_graph.add_edge(new_names[v], new_names[followed]);
                }
            }

            component_graphs.push(component_graph);
            name_mapping.push(original_names);
        }

        Self {
            graphs: component_graphs,
            name_mapping,
        }
    }
}

/// Checks if `graph` contains a cycle. If it does not, returns `None`.
/// If it does, returns `Some(edge)`, where `edge` is an arbitrary edge in a cycle in `graph`.
pub fn cycle_edge(graph: &Graph) -> Option<(usize, usize)> {
    let mut came_from = vec![None; graph.vertex_count()];
    let mut queue = Vec::new();
    queue.push(0);
    came_from[0] = Some(0);

    while let Some(vertex) = queue.pop() {
        for neighbor in graph.neighbors(vertex) {
            if came_from[neighbor].is_some() && came_from[vertex].unwrap() != neighbor {
                return Some((vertex, neighbor));
            }

            if came_from[neighbor].is_none() {
                queue.push(neighbor);
                came_from[neighbor] = Some(vertex);
            }
        }
    }

    None
}
