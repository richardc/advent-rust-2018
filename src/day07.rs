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

    fn lexical_topological_scheduler(&self, elves: usize, overhead: usize) -> usize {
        let mut deps: HashMap<Node, HashSet<Node>> = HashMap::new();
        for edge in &self.edges {
            deps.entry(edge.0).or_default();
            deps.entry(edge.1).or_default().insert(edge.0);
        }

        let mut done: HashSet<Node> = HashSet::new();
        let mut working: HashMap<Node, usize> = HashMap::new();
        let mut time = 0;

        loop {
            let startable = deps
                .iter()
                .filter_map(|(&n, deps)| {
                    if !working.contains_key(&n)
                        && !done.contains(&n)
                        && deps.iter().all(|d| done.contains(d))
                    {
                        Some(n)
                    } else {
                        None
                    }
                })
                .sorted();

            for job in startable {
                if working.len() == elves {
                    break;
                }
                let duration = overhead + job as usize - b'A' as usize + 1; // 'A' == 1...
                working.insert(job, time + duration);
            }

            if working.is_empty() {
                return time;
            }

            // Find the elf that finishes next, and set the time to then
            let soonest = *working.values().min().unwrap();
            time = soonest;

            // Work is done, housekeeping, then reschedule
            working.iter().for_each(|(&k, &v)| {
                if v == time {
                    done.insert(k);
                }
            });
            working.retain(|_, &mut v| v != time);
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

#[aoc(day7, part2)]
fn solve2(graph: &Graph) -> usize {
    graph.lexical_topological_scheduler(5, 60)
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(
        generate(include_str!("day07_example.txt")).lexical_topological_scheduler(2, 0),
        15
    );
}
