#[derive(Clone, Debug)]
pub struct Graph {
    edges: Vec<Vec<Edge>>,
    weights: Vec<f64>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EdgeDir {
    Forward,
    Backward,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Edge {
    dir: EdgeDir,
    second: usize,
}

impl Edge {
    pub fn forward(second: usize) -> Self {
        Self {
            second,
            dir: EdgeDir::Forward,
        }
    }

    pub fn backward(second: usize) -> Self {
        Self {
            second,
            dir: EdgeDir::Backward,
        }
    }
}

impl Graph {
    pub fn new(weights: Vec<f64>) -> Graph {
        Graph {
            edges: vec![vec![]; weights.len()],
            weights,
        }
    }

    pub fn with_edges(weights: Vec<f64>, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(weights);
        for &(from, to) in edges {
            graph.add_edge(from, to);
        }

        graph
    }

    pub fn vertex_count(&self) -> usize {
        self.weights.len()
    }

    pub fn directed_neighbors(&self, vertex: usize, direction: EdgeDir) -> Vec<usize> {
        self.edges[vertex]
            .iter()
            .copied()
            .filter(|e| e.dir == direction)
            .map(|e| e.second)
            .collect()
    }

    pub fn following(&self, vertex: usize) -> Vec<usize> {
        self.directed_neighbors(vertex, EdgeDir::Forward)
    }

    pub fn followers(&self, vertex: usize) -> Vec<usize> {
        self.directed_neighbors(vertex, EdgeDir::Backward)
    }

    pub fn ends(&self, dir: EdgeDir) -> Vec<usize> {
        self.edges
            .iter()
            .enumerate()
            .filter(|(_, edges)| edges.iter().filter(|e| e.dir == dir).count() == 0)
            .map(|(idx, _)| idx)
            .collect()
    }

    pub fn followerless(&self) -> Vec<usize> {
        self.ends(EdgeDir::Backward)
    }

    pub fn followless(&self) -> Vec<usize> {
        self.ends(EdgeDir::Forward)
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(Edge::forward(to));
        self.edges[to].push(Edge::backward(from));
    }

    pub fn neighbors(&self, vertex: usize) -> Vec<usize> {
        self.edges[vertex].iter().map(|e| e.second).collect()
    }

    pub fn weight(&self, vertex: usize) -> f64 {
        self.weights[vertex]
    }

    pub fn copy_without_edge(&self, from: usize, to: usize) -> Graph {
        let mut new_edges = self.edges.clone();
        new_edges[from].retain(|&e| e.second != to || e.dir != EdgeDir::Forward);
        new_edges[to].retain(|&e| e.second != from || e.dir != EdgeDir::Backward);

        Graph {
            edges: new_edges,
            weights: self.weights.clone(),
        }
    }

    pub fn copy_without_vertex(&self, vertex: usize) -> Graph {
        let mut new_edges = self.edges.clone();
        new_edges.remove(vertex);

        for edges in &mut new_edges {
            edges.retain(|&x| x.second != vertex);
        }

        let mut new_weights = self.weights.clone();
        new_weights.remove(vertex);

        Graph {
            edges: new_edges,
            weights: new_weights,
        }
    }
}
