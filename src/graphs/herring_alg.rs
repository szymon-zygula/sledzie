use super::{
    graph::Graph,
    traversal::{cycle_edge, ConnectedComponents},
};
use std::{collections::LinkedList, collections::VecDeque, rc::Rc};

#[derive(Clone)]
struct LinkedTree {
    pub val: Option<usize>,
    pub children: Vec<Rc<LinkedTree>>,
}

impl LinkedTree {
    pub fn new() -> Self {
        LinkedTree {
            val: None,
            children: Vec::new(),
        }
    }

    pub fn from(val: Option<usize>, children: Vec<Rc<LinkedTree>>) -> Self {
        Self { val, children }
    }

    pub fn collapse(&self) -> LinkedList<usize> {
        let mut nodes = LinkedList::new();
        for child in &self.children {
            nodes.append(&mut child.collapse());
        }

        if let Some(val) = self.val {
            nodes.push_back(val);
        }

        nodes
    }
}

#[derive(Debug, Clone)]
pub struct HerringResult {
    weight: f64,
    vertices: LinkedList<usize>,
}

pub fn herring_alg(graph: &Graph) -> HerringResult {
    let components = ConnectedComponents::new(graph);
    let mut result = HerringResult {
        weight: 0.0,
        vertices: LinkedList::new(),
    };

    for i in 0..components.graphs.len() {
        let component_result = herring_comp(&components.graphs[i]);
        result.weight += component_result.weight;
        result.vertices.append(
            &mut component_result
                .vertices
                .iter()
                .map(|&v| components.name_mapping[i][v])
                .collect(),
        );
    }

    result
}

fn herring_comp(graph: &Graph) -> HerringResult {
    if let Some(cycle_edge) = cycle_edge(graph) {
        herring_comp_with_cycle(graph, cycle_edge)
    } else {
        herring_comp_without_cycle(graph)
    }
}

fn herring_comp_with_cycle(graph: &Graph, cycle_edge: (usize, usize)) -> HerringResult {
    HerringResult {
        weight: 0.0,
        vertices: LinkedList::new(),
    }
}

fn rc_vec(n: usize) -> Vec<Rc<LinkedTree>> {
    let mut vec = Vec::with_capacity(n);

    for _ in 0..n {
        vec.push(Rc::new(LinkedTree::new()));
    }

    vec
}

fn herring_comp_without_cycle(graph: &Graph) -> HerringResult {
    let n = graph.vertex_count();
    let mut u = rc_vec(n);
    let mut u_p = rc_vec(n);
    let mut a = vec![0.0; n];
    let mut a_p = a.clone();
    let mut q: VecDeque<_> = graph.followerless().into_iter().collect();
    let mut visited = vec![false; n];
    let mut followers_done = vec![0; n];
    let mut follower_count = vec![0; n];

    for (i, count) in follower_count.iter_mut().enumerate() {
        *count = graph.followers(i).len();
    }

    // graph.followless() should contain exactly one vertex
    let followless = graph.followless()[0];

    while let Some(v) = q.pop_front() {
        if visited[v] {
            continue;
        }

        if follower_count[v] != followers_done[v] {
            q.push_back(v);
            continue;
        }

        visited[v] = true;

        let followings = graph.following(v);

        // In practice at most one following should be found
        for following in followings {
            followers_done[following] += 1;
            q.push_back(following);
        }

        let followers = graph.followers(v);

        // Leaf
        if followers.is_empty() {
            // u_p and a_p were initialized with correct values at the beginning
            u[v] = Rc::new(LinkedTree::from(Some(v), Vec::new()));
            a[v] = graph.weight(v);
            continue;
        }

        u_p[v] = Rc::new(LinkedTree::from(
            None,
            followers
                .iter()
                .map(|&child| Rc::clone(&u[child]))
                .collect::<Vec<_>>(),
        ));

        a_p[v] = followers.iter().map(|&child| a[child]).sum();

        let aa = graph.weight(v) + followers.iter().map(|&follower| a_p[follower]).sum::<f64>();
        if a_p[v] > aa {
            u[v] = Rc::clone(&u_p[v]);
            a[v] = a_p[v];
        } else {
            u[v] = Rc::new(LinkedTree::from(
                Some(v),
                followers
                    .iter()
                    .map(|&child| Rc::clone(&u_p[child]))
                    .collect::<Vec<_>>(),
            ));
            a[v] = aa;
        }
    }

    HerringResult {
        weight: a[followless],
        vertices: u[followless].collapse(),
    }
}
