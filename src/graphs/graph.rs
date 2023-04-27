pub struct Graph {
    edges: Vec<Vec<usize>>,
    weights: Vec<f64>,
}

impl Graph {
    pub fn new(weights: Vec<f64>) -> Graph {
        Graph {
            edges: vec![vec![]; weights.len() + 1],
            weights,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len() - 1
    }

    pub fn add_edge(&mut self, vertex1: usize, vertex2: usize) {
        self.edges[vertex1].push(vertex2);
    }

    pub fn neighbors(&self, vertex: usize) -> &[usize] {
        &self.edges[vertex]
    }

    pub fn weight(&self, vertex: usize) -> f64 {
        self.weights[vertex]
    }

    pub fn copy_without_edge(&self, vertex1: usize, vertex2: usize) -> Graph {
        let mut new_edges = self.edges.clone();
        new_edges[vertex1].retain(|&x| x != vertex2);
        Graph {
            edges: new_edges,
            weights: self.weights.clone(),
        }
    }

    pub fn copy_without_vertex(&self, vertex: usize) -> Graph {
        let mut new_edges = self.edges.clone();
        new_edges.remove(vertex);
        for i in 0..new_edges.len() {
            new_edges[i].retain(|&x| x != vertex);
        }
        let mut new_weights = self.weights.clone();
        new_weights.remove(vertex);
        Graph {
            edges: new_edges,
            weights: new_weights,
        }
    }
}
