use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Node = char;

#[derive(Debug)]
struct Edge(Node, Node);

impl std::str::FromStr for Edge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_ascii_whitespace().collect_vec();
        Ok(Edge(
            toks[1].chars().next().unwrap(),
            toks[7].chars().next().unwrap(),
        ))
    }
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Edge>,
}

impl std::str::FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s.lines().map(|l| l.parse().unwrap()).collect::<Vec<Edge>>();
        Ok(Graph { edges })
    }
}

impl Graph {
    fn lexical_topological(&self) -> Vec<Node> {
        let mut deps: HashMap<Node, HashSet<Node>> = HashMap::new();
        for edge in &self.edges {
            deps.entry(edge.0).or_default();
            deps.entry(edge.1).or_default().insert(edge.0);
        }

        let mut done: HashSet<Node> = HashSet::new();
        let mut order = vec![];
        loop {
            if let Some(next) = deps
                .iter()
                .filter_map(|(&n, deps)| {
                    if !done.contains(&n) && deps.iter().all(|d| done.contains(d)) {
                        Some(n)
                    } else {
                        None
                    }
                })
                .sorted()
                .next()
            {
                done.insert(next);
                order.push(next);
            } else {
                return order;
            }
        }
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Graph {
    input.parse().unwrap()
}

#[aoc(day7, part1)]
fn solve(graph: &Graph) -> String {
    String::from_iter(graph.lexical_topological())
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(
        solve(&generate(include_str!("day07_example.txt"))),
        "CABDFE"
    )
}
