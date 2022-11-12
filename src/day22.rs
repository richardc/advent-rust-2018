use std::collections::HashMap;

use itertools::{iproduct, Itertools};

#[derive(Default, Clone)]
struct Cave {
    depth: usize,
    target: (usize, usize),
    geologic: HashMap<(usize, usize), usize>,
}

impl std::str::FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let depth = lines[0].strip_prefix("depth: ").unwrap().parse()?;
        let (x, y) = lines[1]
            .strip_prefix("target: ")
            .unwrap()
            .split_once(',')
            .unwrap();
        Ok(Self {
            depth,
            target: (x.parse()?, y.parse()?),
            ..Default::default()
        })
    }
}

impl Cave {
    fn geologic(&mut self, x: usize, y: usize) -> usize {
        if let Some(value) = self.geologic.get(&(x, y)) {
            return *value;
        }
        let value = match (x, y) {
            (0, 0) => 0,
            (x, y) if (x, y) == self.target => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => self.erosion(x - 1, y) * self.erosion(x, y - 1),
        };
        self.geologic.insert((x, y), value);
        value
    }

    fn erosion(&mut self, x: usize, y: usize) -> usize {
        (self.geologic(x, y) + self.depth) % 20183
    }

    fn region(&mut self, x: usize, y: usize) -> usize {
        self.erosion(x, y) % 3
    }

    fn risk_level(&mut self) -> usize {
        iproduct!(0..=self.target.0, 0..=self.target.1)
            .map(|(x, y)| self.region(x, y))
            .sum()
    }
}

#[aoc_generator(day22)]
fn generate(input: &str) -> Cave {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
fn solve(cave: &Cave) -> usize {
    (*cave).clone().risk_level()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day22_example.txt"))), 114)
}
