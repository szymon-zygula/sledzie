use super::{
    graph::Graph,
    traversal::{connected_components, cycle_edge, leaves},
};
use std::{collections::LinkedList, rc::Rc};

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
        let Some(val) = self.val else { return LinkedList::new() };

        let mut nodes = LinkedList::new();
        nodes.push_back(val);

        for child in &self.children {
            nodes.append(&mut child.collapse());
        }

        nodes
    }
}

pub struct HerringResult {
    weight: f64,
    vertices: LinkedList<usize>,
}

pub fn herring_alg(graph: &Graph) -> HerringResult {
    let components = connected_components(graph);
    let mut result = HerringResult {
        weight: 0.0,
        vertices: LinkedList::new(),
    };

    for component in components {
        let mut component_result = herring_comp(graph, component);
        result.weight += component_result.weight;
        result.vertices.append(&mut component_result.vertices);
    }

    result
}

fn herring_comp(graph: &Graph, component: usize) -> HerringResult {
    if let Some(cycle_edge) = cycle_edge(graph, component) {
        herring_comp_with_cycle(graph, component, cycle_edge)
    } else {
        herring_comp_without_cycle(graph, component)
    }
}

fn herring_comp_with_cycle(
    graph: &Graph,
    component: usize,
    cycle_edge: (usize, usize),
) -> HerringResult {
    HerringResult {
        weight: 0.0,
        vertices: LinkedList::new(),
    }
}

fn herring_comp_without_cycle(graph: &Graph, component: usize) -> HerringResult {
    let n = graph.vertex_count();
    let mut u = vec![Rc::new(LinkedTree::new()); n];
    let mut u_p = u.clone();
    let mut a = vec![0.0; n];
    let mut a_p = a.clone();
    let mut q = leaves(graph, component);

    while let Some(v) = q.pop() {
        let parents = graph.parents(v);

        // In practice only one parent should be found
        for parent in parents {
            q.push(parent);
        }

        let children = graph.children(v);

        if children.is_empty() {
            // u_p and a_p were initialized with correct values
            u[v] = Rc::new(LinkedTree::from(Some(v), Vec::new()));
            a[v] = graph.weight(v);
            continue;
        }

        u_p[v] = Rc::new(LinkedTree::from(
            None,
            children
                .iter()
                .map(|&child| Rc::clone(&u[child]))
                .collect::<Vec<_>>(),
        ));

        a_p[v] = children.iter().map(|&child| a[child]).sum();

        let aa = graph.weight(v) + children.iter().map(|&child| a_p[child]).sum::<f64>();
        if a_p[v] > aa {
            u[v] = Rc::clone(&u_p[v]);
            a[v] = a_p[v];
        } else {
            u[v] = Rc::new(LinkedTree::from(
                Some(v),
                children
                    .iter()
                    .map(|&child| Rc::clone(&u_p[child]))
                    .collect::<Vec<_>>(),
            ));
            a[v] = aa;
        }
    }

    HerringResult {
        weight: a[component],
        vertices: u[component].collapse(),
    }
}
