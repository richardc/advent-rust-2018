use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Clone)]
struct Plants {
    steps: usize,
    pots: Vec<bool>,
    rules: HashMap<[bool; 5], bool>,
}

impl std::fmt::Debug for Plants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(self.pots.iter().map(|&p| if p { '#' } else { '.' }))
        )
    }
}

impl std::str::FromStr for Plants {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let start = lines.next().unwrap();
        let (_, pots) = start.split_once(": ").unwrap();
        let pots = pots.chars().map(|c| c == '#').collect();
        let rules: HashMap<[bool; 5], bool> = lines
            .skip(1)
            .map(|l| {
                let (pattern, state) = l.split_once(" => ").unwrap();
                (
                    pattern
                        .chars()
                        .map(|c| c == '#')
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                    state.chars().next().unwrap() == '#',
                )
            })
            .collect();

        let steps = 0;
        Ok(Plants { steps, pots, rules })
    }
}

impl Plants {
    fn step(&mut self) {
        let pots = [&[false; 3][..], &self.pots, &[false; 3][..]].concat();
        let next = pots
            .windows(5)
            .map(|w| match self.rules.get(w) {
                Some(&x) => x,
                None => false,
            })
            .collect();
        self.steps += 1;
        self.pots = next;
    }

    fn score(&self) -> i64 {
        self.pots
            .iter()
            .enumerate()
            .map(|(i, &p)| if p { i as i64 - self.steps as i64 } else { 0 })
            .sum()
    }
}

#[aoc_generator(day12)]
fn generate(input: &str) -> Plants {
    input.parse().unwrap()
}

#[aoc(day12, part1)]
fn solve(plants: &Plants) -> i64 {
    let mut plants = plants.clone();
    // println!("{:?}", plants);
    for _ in 0..20 {
        plants.step();
        // println!("{:?}", plants);
    }
    plants.score()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day12_example.txt"))), 325);
}

#[aoc(day12, part2)]
fn solve2(plants: &Plants) -> i64 {
    let mut plants = plants.clone();
    let goal = 50_000_000_000;
    let mut diffs = VecDeque::from([0; 5]);
    let mut last = plants.score();
    for step in 1.. {
        plants.step();
        let score = plants.score();
        let diff = score - last;
        last = score;
        diffs.pop_front();
        diffs.push_back(diff);
        if diffs.iter().all(|&x| x == diff) {
            // We've stabilised into a linear relationship,
            // so now we can just calculate
            return (goal - step) * diff + score;
        }
    }
    unreachable!()
}
